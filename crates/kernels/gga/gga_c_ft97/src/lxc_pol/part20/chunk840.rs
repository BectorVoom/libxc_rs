//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 840/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk840<F: Float>(t25413: F, t2876: F, t25412: F, t1477: F, t2892: F, t193: F, t6210: F, t6213: F, t6261: F, t880: F, t25135: F, t798: F, t317: F, t1466: F, t1506: F, t24917: F, t24945: F, t25214: F, t25378: F, t25389: F, t25393: F, t25397: F, t25402: F, t25406: F, t25410: F, t2649: F, t2745: F, t301: F, t6216: F, t6263: F, t6267: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t25414 = t25413 * t2876;
    let t25415 = t25412 * t25414;
    let t25426 = t1477 * t2892;
    let t25427 = t193 * t25426;
    let t25430 = t6210 * t6213;
    let t25434 = t6261 * t880;
    let t25435 = t193 * t25434;
    let t25438 = t798 * t25135;
    let t25439 = t25438 * t317;
    let t25440 = t193 * t25439;
    let t25443 = -t301 * t25389 + t1466 * t25393 - 2.0 / 3.0 * t1466 * t25397 - t1466 * t25402 / 3.0 - 2.0 / 3.0 * t1466 * t25406 + 2.0 / 9.0 * t25410 + 2.0 / 9.0 * t6216 * t25415 + t6210 * t6267 / 3.0 + t6210 * t6263 / 3.0 + 4.0 * t25214 + 8.0 * t24945 + 8.0 * t24917 - 12.0 * t25378 + t1466 * t25427 / 6.0 - t25430 / 9.0 - t2745 * t1506 - t2649 * t1506 + t1466 * t25435 / 3.0 + t1466 * t25440 / 6.0;
    (t25414, t25415, t25426, t25427, t25430, t25434, t25435, t25438, t25439, t25440, t25443)
}
