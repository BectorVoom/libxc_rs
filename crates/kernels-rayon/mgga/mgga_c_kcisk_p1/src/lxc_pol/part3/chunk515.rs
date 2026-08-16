//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 515/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk515(t1328: f64, t4158: f64, t1220: f64, t3729: f64, t3774: f64, t3780: f64, t3789: f64, t3793: f64, t3801: f64, t3807: f64, t3808: f64, t3810: f64, t3910: f64, t3917: f64, t3920: f64, t3925: f64, t3930: f64, t412: f64) -> (f64, f64) {
    let t4159 = t4158 * t1328;
    let t4162 = 0.22109259259259259258e-2_f64 * t3774 - 0.55273148148148148147e-3_f64 * t3780 + 0.49745833333333333332e-2_f64 * t3789 + 0.13265555555555555555e-1_f64 * t3793 - 0.33163888888888888888e-2_f64 * t3801 + t3729 * t412 - t3807 - 0.88437037037037037034e-2_f64 * t3808 + 0.33163888888888888888e-2_f64 * t3810 + 0.24872916666666666666e-2_f64 * t3910 + 0.24320185185185185185e-1_f64 * t3917 - 0.13265555555555555555e-1_f64 * t3920 + 0.193e0_f64 * t1220 * t3925 + 0.74498e-1_f64 * t3930 * t3925 - 0.193e0_f64 * t1220 * t4159;
    (t4159, t4162)
}
