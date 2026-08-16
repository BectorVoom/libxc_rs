//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 789/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk789(t13728: f64, t1445: f64, t597: f64, t12054: f64, t3377: f64, t1457: f64, t1572: f64, t12068: f64, t874: f64, t1562: f64, t13750: f64, t531: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13805 = t1445 * t13728;
    let t13806 = t597 * t13805;
    let t13808 = t12054 * t3377;
    let t13810 = t1457 * t13728;
    let t13811 = t1572 * t13810;
    let t13813 = t12068 * t874;
    let t13814 = t1445 * t13813;
    let t13815 = t1562 * t13814;
    let t13818 = t531 * t13750;
    (t13805, t13806, t13808, t13810, t13811, t13813, t13814, t13815, t13818)
}
