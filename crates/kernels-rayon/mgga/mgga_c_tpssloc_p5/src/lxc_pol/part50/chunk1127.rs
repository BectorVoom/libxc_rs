//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1127/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1127(t111: f64, t7758: f64, t112: f64, t26509: f64, t25: f64, t40772: f64, t1519: f64, t213: f64, t225: f64, t794: f64, t25051: f64, t1509: f64, t6624: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86647 = t7758 * t111;
    let t86656 = t26509 * t112;
    let t86716 = t40772 * t25;
    let t86873 = t213 * t1519 * t225;
    let t86893 = t794 * t1519;
    let t86988 = t25051 * t225;
    let t87567 = t6624 * t1509;
    (t86647, t86656, t86716, t86873, t86893, t86988, t87567)
}
