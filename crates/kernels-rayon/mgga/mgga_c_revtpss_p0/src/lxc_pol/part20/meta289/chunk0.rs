//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1156/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1156(t12123: f64, t3318: f64, t1043: f64, t3153: f64, t3133: f64, t4982: f64, t1071: f64, t1089: f64, t999: f64, t3046: f64, t3286: f64, t3057: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12128 = t12123 * t3318;
    let t12131 = t1043 * t3153;
    let t12132 = t4982 * t3133;
    let t12133 = t12131 * t12132;
    let t12137 = t1071 * t3133 * t1089;
    let t12143 = t999 * t3133 * t1089;
    let t12146 = t3046 * t3286;
    let t12149 = t3057 * t3286;
    (t12128, t12131, t12132, t12133, t12137, t12143, t12146, t12149)
}
