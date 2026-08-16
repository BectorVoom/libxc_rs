//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 772/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk772(t1926: f64, t23326: f64, t344: f64, t381: f64, t225: f64, t1054: f64, t883: f64, t1065: f64, t607: f64, t6733: f64, t6691: f64, t1955: f64, t3175: f64) -> (f64, f64, f64, f64) {
    let t23327 = t1926 * t23326;
    let t23328 = t344 * t381;
    let t23329 = t23328 * t225;
    let t23330 = t1054 * t883;
    let t23331 = t607 * t1065;
    let t23332 = t23330 * t23331;
    let t23333 = t23329 * t23332;
    let t23336 = t6733 * t381;
    let t23337 = t23336 * t6691;
    let t23340 = t1955 * t3175;
    (t23327, t23333, t23337, t23340)
}
