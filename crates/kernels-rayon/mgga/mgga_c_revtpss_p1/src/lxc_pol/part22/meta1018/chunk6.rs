//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3527/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3527(t15794: f64, t15926: f64, t1011: f64, t15993: f64, t18937: f64, t11875: f64, t15785: f64, t15906: f64, t16081: f64, t19450: f64, t19639: f64, t20089: f64, t3117: f64, t42571: f64, t43279: f64, t4912: f64, t4915: f64, t53586: f64, t54623: f64, t54638: f64, t54646: f64, t54648: f64, t54916: f64, t6263: f64, t6271: f64, t63297: f64) -> f64 {
    let t66814 = t15926 * t15794;
    let t66822 = t1011 * t15993 * t18937;
    let t66827 = 0.11433071498151929859e-2_f64 * t54623 - 0.12862205435420921092e-2_f64 * t15906 * t3117 * t19450 * t15785 + 0.85748036236139473944e-3_f64 * t11875 * t3117 * t20089 * t19639 + 0.42874018118069736972e-3_f64 * t11875 * t3117 * t6271 * t53586 + 0.12862205435420921092e-2_f64 * t16081 * t3117 * t19450 * t43279 + 0.45732285992607719436e-2_f64 * t54916 * t4912 - 0.57165357490759649296e-3_f64 * t66814 + 0.6351706387862183255e-3_f64 * t54638 + 0.30488190661738479624e-2_f64 * t42571 * t6263 - 0.96545937095505185476e-2_f64 * t54646 - 0.10162730220579493208e-2_f64 * t54648 + t66822 / 324.0_f64 - t1011 * t4915 * t63297 / 144.0_f64;
    t66827
}
