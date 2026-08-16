//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1256/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1256(t24565: f64, t25939: f64, t25749: f64, t3907: f64, t3909: f64, t25032: f64, t953: f64, t155: f64, t24088: f64, t24498: f64, t24559: f64, t24569: f64, t25175: f64, t25920: f64, t25929: f64, t25932: f64, t25935: f64, t2668: f64, t2754: f64, t2761: f64, t2797: f64, t313: f64, t314: f64, t329: f64, t7984: f64, t8027: f64, t8149: f64, t8168: f64, t877: f64, t914: f64, t930: f64, t935: f64) -> f64 {
    let t25940 = t25939 * t24565;
    let t25946 = t3907 * t25749 * t3909;
    let t25962 = t953 * t25032;
    let t25964 = 0.24727214904288022343e1_f64 * t2797 * t8027 - 0.1343485452223045261e0_f64 * t25920 + 0.13909058383662012568e1_f64 * t930 * t914 * t25175 + 0.50380704458364197288e-2_f64 * t953 * t24559 + 0.4707813348935102208e4_f64 * t25929 * t2754 - 0.1569271116311700736e4_f64 * t25932 * t2761 - 0.24951672488470492992e3_f64 * t25935 + 0.23184437530160156653e8_f64 * t25940 * t313 * t24569 * t935 + 0.18583473745796456084e3_f64 * t25946 - 0.12117441361606500412e2_f64 * t8149 * t7984 - 0.12117441361606500412e2_f64 * t8149 * t8168 + 0.9291736872898228042e2_f64 * t3907 * t24498 * t3909 - 0.30972456242994093473e2_f64 * t2668 * t24498 * t877 + 0.15599358861923136642e2_f64 * t155 * t329 * t24088 * t314 - 0.11195712101858710508e-1_f64 * t25962;
    t25964
}
