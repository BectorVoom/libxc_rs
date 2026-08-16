//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1271/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1271(t1881: f64, t3228: f64, t1901: f64, t3237: f64, t1008: f64, t6102: f64, t1137: f64, t5590: f64, t1089: f64, t12610: f64, t14022: f64, t18072: f64, t18079: f64, t18085: f64, t18087: f64, t1849: f64, t1899: f64, t21771: f64, t3266: f64, t386: f64, t387: f64, t418: f64, t435: f64) -> f64 {
    let t23480 = t3228 * t1881;
    let t23482 = t3237 * t1901;
    let t23484 = t3228 * t1901;
    let t23486 = t1008 * t6102;
    let t23494 = t1137 * t5590;
    let t23499 = 0.20007875121765877254e-2_f64 * t14022 + 0.68598428988911579156e-2_f64 * t418 * t1089 * t12610 * t1849 - 0.85748036236139473944e-3_f64 * t418 * t386 * t3266 * t1899 + 0.12862205435420921092e-2_f64 * t23480 + 0.40015750243531754508e-2_f64 * t23482 - 0.42874018118069736972e-3_f64 * t23484 - 0.85748036236139473944e-3_f64 * t23486 - 0.42874018118069736972e-3_f64 * t418 * t386 * t387 * t435 * t21771 - 0.16006300097412701803e-1_f64 * t18072 + 7.0_f64 / 36.0_f64 * t23494 + 7.0_f64 / 6.0_f64 * t18079 + 7.0_f64 / 24.0_f64 * t18085 + 35.0_f64 / 18.0_f64 * t18087;
    t23499
}
