//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1253/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1253<F: Float>(t11135: F, t5552: F, t2728: F, t8440: F, t16705: F, t3459: F, t24303: F, t977: F, t10805: F, t5559: F, t841: F, t1052: F, t22139: F) -> (F, F, F, F, F, F) {
    let t32708 = F::new(4.0) * t5552 * t11135;
    let t32713 = F::new(2.0) * t8440 * t2728;
    let t32715 = F::new(2.0) * t16705 * t3459;
    let t32716 = t24303 * t977;
    let t32719 = F::new(12.0) * t5559 * t10805 * t841;
    let t32720 = t22139 * t1052;
    (t32708, t32713, t32715, t32716, t32719, t32720)
}
