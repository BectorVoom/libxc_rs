//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1699/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1699(t225: f64, t567: f64, t6434: f64, t214: f64, t1985: f64, t6460: f64, t6906: f64, t6889: f64, t6347: f64, t6890: f64, t6888: f64, t26193: f64, t7691: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28199 = t6434 * t225 * t567;
    let t28200 = t214 * t28199;
    let t28201 = t1985 * t28200;
    let t28205 = t6906 * t6460;
    let t28206 = t6889 * t28205;
    let t28207 = t1985 * t28206;
    let t28209 = t6890 * t6347;
    let t28210 = t6889 * t28209;
    let t28211 = t6888 * t28210;
    let t28213 = t26193 * t7691;
    (t28199, t28200, t28201, t28205, t28206, t28207, t28209, t28210, t28211, t28213)
}
