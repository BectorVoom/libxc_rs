//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1142/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1142<F: Float>(t1008: F, t5950: F, t1036: F, t1089: F, t1734: F, t175: F, t864: F, t5561: F, t997: F, t1032: F, t5971: F, t1016: F, t1018: F, t1026: F, t174: F, t18121: F, t18139: F, t18141: F, t18147: F, t1879: F, t20092: F, t21901: F, t3266: F, t386: F, t387: F, t418: F, t5679: F) -> (F,) {
    let t23530 = t1008 * t5950;
    let t23553 = t1036 * t1089 * t175 * t1734 * t864;
    let t23556 = t997 * t5561;
    let t23558 = t1032 * t5971;
    let t23563 = 0.17149607247227894789e-1 * t23530 + 0.85748036236139473944e-2 * t418 * t1026 * t387 * t174 * t20092 + 0.25724410870841842183e-2 * t418 * t386 * t5679 * t1018 + 0.25724410870841842184e-2 * t418 * t386 * t3266 * t1879 + 0.25724410870841842184e-2 * t418 * t386 * t387 * t1016 * t21901 - 0.17149607247227894789e-2 * t23553 + 0.51448821741683684367e-2 * t18121 - 0.80031500487063509015e-1 * t23556 - 0.80031500487063509014e-2 * t23558 + 35.0 / 54.0 * t18139 + 7.0 / 36.0 * t18141 + 7.0 / 24.0 * t18147;
    (t23563,)
}
