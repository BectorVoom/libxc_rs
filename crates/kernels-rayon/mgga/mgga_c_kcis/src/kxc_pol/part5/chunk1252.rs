//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1252/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1252(t3734: f64, t6918: f64, t1464: f64, t2011: f64, t5627: f64, t1495: f64, t1468: f64, t5632: f64, t5676: f64, t4123: f64, t5756: f64, t5880: f64) -> (f64, f64, f64, f64, f64) {
    let t20931 = t3734 * t6918;
    let t20932 = t1464 * t20931;
    let t20934 = t5627 * t2011;
    let t20935 = t1495 * t20934;
    let t20936 = t1468 * t20935;
    let t20937 = t1464 * t20936;
    let t20939 = t5632 * t5676;
    let t20940 = t4123 * t20939;
    let t20941 = t1464 * t20940;
    let t20943 = t5756 * t5880;
    (t20932, t20934, t20937, t20941, t20943)
}
