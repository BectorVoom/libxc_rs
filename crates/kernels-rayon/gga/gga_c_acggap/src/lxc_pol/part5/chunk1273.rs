//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1273/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1273(t1008: f64, t5950: f64, t1036: f64, t1089: f64, t1734: f64, t175: f64, t864: f64, t5561: f64, t997: f64, t1032: f64, t5971: f64, t1016: f64, t1018: f64, t1026: f64, t174: f64, t18121: f64, t18139: f64, t18141: f64, t18147: f64, t1879: f64, t20092: f64, t21901: f64, t3266: f64, t386: f64, t387: f64, t418: f64, t5679: f64) -> f64 {
    let t23530 = t1008 * t5950;
    let t23553 = t1036 * t1089 * t175 * t1734 * t864;
    let t23556 = t997 * t5561;
    let t23558 = t1032 * t5971;
    let t23563 = 0.17149607247227894789e-1_f64 * t23530 + 0.85748036236139473944e-2_f64 * t418 * t1026 * t387 * t174 * t20092 + 0.25724410870841842183e-2_f64 * t418 * t386 * t5679 * t1018 + 0.25724410870841842184e-2_f64 * t418 * t386 * t3266 * t1879 + 0.25724410870841842184e-2_f64 * t418 * t386 * t387 * t1016 * t21901 - 0.17149607247227894789e-2_f64 * t23553 + 0.51448821741683684367e-2_f64 * t18121 - 0.80031500487063509015e-1_f64 * t23556 - 0.80031500487063509014e-2_f64 * t23558 + 35.0_f64 / 54.0_f64 * t18139 + 7.0_f64 / 36.0_f64 * t18141 + 7.0_f64 / 24.0_f64 * t18147;
    t23563
}
