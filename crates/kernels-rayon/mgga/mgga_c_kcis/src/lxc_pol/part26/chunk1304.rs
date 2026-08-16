//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1304/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1304(t29270: f64, t4142: f64, t29277: f64, t12832: f64, t29594: f64, t7978: f64, t1610: f64, t30409: f64, t6176: f64, t7509: f64, t21931: f64, t27387: f64, t4153: f64) -> (f64, f64, f64, f64, f64) {
    let t102311 = t4142 * t29270;
    let t102313 = t4142 * t29277;
    let t102318 = t7978 * t12832 * t29594;
    let t102328 = t6176 * t30409 * t7509 * t1610;
    let t102334 = t4153 * t27387 * t21931;
    (t102311, t102313, t102318, t102328, t102334)
}
