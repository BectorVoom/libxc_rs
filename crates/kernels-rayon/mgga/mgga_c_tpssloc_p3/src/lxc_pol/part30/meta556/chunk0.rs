//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1915/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1915(t28205: f64, t6889: f64, t1985: f64, t6347: f64, t6890: f64, t6888: f64, t26193: f64, t7691: f64, t1842: f64, t7749: f64, t3887: f64, t2015: f64, t6439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28206 = t6889 * t28205;
    let t28207 = t1985 * t28206;
    let t28209 = t6890 * t6347;
    let t28210 = t6889 * t28209;
    let t28211 = t6888 * t28210;
    let t28213 = t26193 * t7691;
    let t28214 = t6888 * t28213;
    let t28219 = t7749 * t1842;
    let t28220 = t3887 * t28219;
    let t28223 = t2015 * t6439;
    (t28206, t28207, t28209, t28210, t28211, t28213, t28214, t28220, t28223)
}
