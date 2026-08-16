//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 929/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk929(t13734: f64, t13749: f64, t1266: f64, t1275: f64, t1234: f64, t1264: f64, t13675: f64, t13680: f64, t13683: f64, t13697: f64, t13702: f64, t13705: f64, t13708: f64, t13711: f64, t13717: f64, t374: f64, t4031: f64, t4033: f64, t4081: f64, t4096: f64, t4122: f64, t4130: f64, t45: f64) -> f64 {
    let t13750 = t13734 + t13749;
    let t13752 = t1266 * t13750 * t1275;
    let t13759 = 1.0_f64 * t1234 * t13675 + 0.51725014705706168417e3_f64 * t13680 * t13683 + 0.19751789702565206229e-1_f64 * t45 * t13697 * t374 + 0.48245472966453314466e2_f64 * t4081 * t13702 - 6.0_f64 * t13705 * t4033 + 6.0_f64 * t4081 * t13708 - 6.0_f64 * t4031 * t13711 + 0.1038945353962551798e3_f64 * t1264 * t13717 - 0.58482233974552040708e0_f64 * t1264 * t13752 - 0.17544670192365612213e1_f64 * t4096 * t4122 - 0.51947267698127589899e2_f64 * t4096 * t4130;
    t13759
}
