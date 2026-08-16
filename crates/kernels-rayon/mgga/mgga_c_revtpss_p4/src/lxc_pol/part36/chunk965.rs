//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 965/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk965(t22857: f64, t9994: f64, t1390: f64, t828: f64, t4003: f64, t1370: f64, t13798: f64, t13801: f64, t1410: f64, t22038: f64, t22044: f64, t22057: f64, t22059: f64, t22063: f64, t22069: f64, t22843: f64, t22849: f64, t22854: f64, t4002: f64, t5671: f64, t9735: f64, t9993: f64) -> (f64, f64, f64, f64, f64) {
    let t22858 = t22857 * t9994;
    let t22860 = t1390 * t828 * t22858;
    let t22863 = t22857 * t4003;
    let t22865 = t1390 * t828 * t22863;
    let t22874 = -0.51448821741683684367e-2_f64 * t5671 * t22843 + 7.0_f64 / 48.0_f64 * t22038 - 7.0_f64 / 16.0_f64 * t22044 - t1370 * t22849 / 48.0_f64 - t9735 + 0.12862205435420921092e-1_f64 * t1410 * t22854 - 0.12862205435420921092e-2_f64 * t9993 * t22860 + 0.12862205435420921092e-2_f64 * t4002 * t22865 - 0.15246000842785598468e-3_f64 * t22057 - 0.60023625365297631762e-1_f64 * t22059 + 0.21437009059034868486e-4_f64 * t22063 + 0.76230004213927992338e-3_f64 * t22069 - 35.0_f64 / 72.0_f64 * t13798 + 0.30492001685571196935e-4_f64 * t13801;
    (t22858, t22860, t22863, t22865, t22874)
}
