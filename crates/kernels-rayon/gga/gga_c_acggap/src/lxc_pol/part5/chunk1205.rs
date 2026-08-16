//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1205/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1205(t1421: f64, t301: f64, t15393: f64, t176: f64, t525: f64, t1165: f64, t3456: f64, t4241: f64, t5852: f64, t13371: f64, t13373: f64, t16900: f64, t16902: f64, t16911: f64, t16916: f64, t16921: f64, t16926: f64, t16928: f64, t16930: f64) -> f64 {
    let t21955 = t1421 * t301;
    let t21958 = t15393 * t176 * t525 * t21955;
    let t21970 = t3456 * t1165 * t5852 * t4241;
    let t21972 = -0.34299214494455789578e-1_f64 * t16900 + 0.12004725073059526352e-1_f64 * t16902 + 0.17149607247227894789e-1_f64 * t21958 - 0.40015750243531754508e-2_f64 * t13371 - 0.12004725073059526352e-1_f64 * t13373 - 0.85748036236139473944e-3_f64 * t16911 - 0.17149607247227894789e-2_f64 * t16916 - 0.17149607247227894789e-2_f64 * t16921 - 0.85748036236139473944e-3_f64 * t16926 - 0.42874018118069736972e-3_f64 * t16928 - 7.0_f64 / 36.0_f64 * t16930 - 0.25724410870841842183e-2_f64 * t21970;
    t21972
}
