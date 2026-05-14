//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1116/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1116<F: Float>(t11135: F, t5552: F, t2728: F, t8440: F, t16705: F, t3459: F, t24303: F, t977: F, t10805: F, t5559: F, t841: F, t1052: F, t22139: F, t1960: F, t2208: F, t31458: F, t31461: F, t31463: F, t31465: F, t31468: F, t31469: F, t31470: F, t31472: F, t31474: F, t31476: F, t32111: F, t32155: F, t32202: F, t32248: F, t32290: F, t32344: F, t32386: F, t32424: F, t32467: F, t32508: F, t32527: F, t32565: F, t32598: F, t32636: F, t32665: F, t32701: F, t331: F, t3511: F) -> (F,) {
    let t32708 = 4.0 * t5552 * t11135;
    let t32713 = 2.0 * t8440 * t2728;
    let t32715 = 2.0 * t16705 * t3459;
    let t32716 = t24303 * t977;
    let t32719 = 12.0 * t5559 * t10805 * t841;
    let t32720 = t22139 * t1052;
    let t32721 = t31458 + (t32111 + t32155 + t32202 + t32248 + t32290 + t32344 + t32386 + t32424 + t32467 + t32508 + t32527 + t32565 + t32598 + t32636 + t32665 + t32701) * t331 + t31461 + t32708 + t31463 + 2.0 * t1960 * t3511 * t2208 - t32713 + t32715 - t31465 - t32716 - t31468 - t32719 + t31469 + t31470 - t31472 - t32720 + t31474 - t31476;
    (t32721,)
}
