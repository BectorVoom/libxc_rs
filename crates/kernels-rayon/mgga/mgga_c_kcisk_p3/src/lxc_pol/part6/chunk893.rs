//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 893/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk893(t1421: f64, t16885: f64, t16897: f64, t22646: f64, t22652: f64, t22654: f64, t22656: f64, t28886: f64, t28894: f64, t28898: f64, t28902: f64, t28906: f64, t456: f64) -> f64 {
    let t28909 = -0.98556445e-3_f64 * t16885 - 0.43802864444444444445e-3_f64 * t16897 - 0.36958666875e-3_f64 * t456 * t28886 + 0.21901432222222222222e-2_f64 * t22646 - 0.26281718666666666667e-2_f64 * t22652 + 0.13140859333333333334e-2_f64 * t22654 - 0.59133867e-2_f64 * t22656 + 0.29201909629629629629e-2_f64 * t1421 * t28894 + 0.59133867e-2_f64 * t1421 * t28898 + 0.39422577999999999999e-2_f64 * t1421 * t28902 - 0.65704296666666666666e-2_f64 * t1421 * t28906;
    t28909
}
