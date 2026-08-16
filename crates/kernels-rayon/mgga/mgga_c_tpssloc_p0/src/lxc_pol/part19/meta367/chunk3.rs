//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1347/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1347(t2986: f64, t2990: f64, t43057: f64, t10325: f64, t2987: f64, t3008: f64, t4509: f64, t13797: f64, t984: f64, t10216: f64, t343: f64, t9288: f64) -> (f64, f64, f64, f64, f64) {
    let t43059 = t2986 * t43057 * t2990;
    let t43061 = t2987 * t10325;
    let t43065 = t4509 * t3008;
    let t43069 = t13797 * t984;
    let t43070 = t343 * t10216;
    let t43071 = t43070 * t9288;
    (t43059, t43061, t43065, t43069, t43071)
}
