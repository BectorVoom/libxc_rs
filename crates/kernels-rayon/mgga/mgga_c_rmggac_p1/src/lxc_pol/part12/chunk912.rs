//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 912/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk912(t236: f64, t3351: f64, t5204: f64, t9188: f64, t3352: f64, t511: f64, t5211: f64, t2004: f64, t38472: f64, t2320: f64, t36315: f64, t7717: f64, t8616: f64) -> (f64, f64, f64, f64, f64) {
    let t39748 = t3351 * t9188 * t236 * t5204;
    let t39752 = t3351 * t3352 * t511 * t5211;
    let t39754 = t38472 * t2004;
    let t39756 = t36315 * t2320;
    let t39758 = t7717 * t8616;
    (t39748, t39752, t39754, t39756, t39758)
}
