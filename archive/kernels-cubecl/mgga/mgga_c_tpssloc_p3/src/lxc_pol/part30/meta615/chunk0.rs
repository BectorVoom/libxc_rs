//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2014/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2014<F: Float>(t23563: F, t6740: F, t10922: F, t6717: F, t3200: F, t83015: F, t1030: F, t1058: F, t3068: F, t25511: F, t6743: F, t23592: F, t23631: F, t974: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t83138 = t6740 * t23563;
    let t83157 = t6717 * t10922;
    let t83215 = t3200 * t83015;
    let t83220 = t1058 * sigma0 * t1030 * t3068;
    let t83233 = t6743 * t25511;
    let t83239 = t23631 * t974 * t23592;
    (t83138, t83157, t83215, t83220, t83233, t83239)
}
