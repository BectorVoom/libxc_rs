//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1214/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1214(t1634: f64, t4707: f64, t6209: f64, t972: f64, t6206: f64, t3014: f64, t6205: f64, t4711: f64, t11509: f64, t6189: f64, t15101: f64, t4595: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19294 = t1634 * t4707;
    let t19297 = t6209 * t972;
    let t19300 = t6206 * t972;
    let t19303 = t6205 * t3014;
    let t19304 = t19303 * t972;
    let t19307 = t4711 * t4707;
    let t19310 = t6189 * t11509;
    let t19311 = t19310 * t972;
    let t19315 = 4.0_f64 * t15101 * t4595;
    (t19294, t19297, t19300, t19304, t19307, t19311, t19315)
}
