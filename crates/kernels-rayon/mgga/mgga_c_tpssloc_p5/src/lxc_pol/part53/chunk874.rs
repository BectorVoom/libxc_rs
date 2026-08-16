//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 874/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk874(t22674: f64, t8621: f64, t6897: f64, t6906: f64, t7213: f64, t6889: f64, t1985: f64, t2085: f64, t214: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31594 = t22674 * t8621;
    let t31595 = t6897 * t31594;
    let t31607 = t6906 * t7213;
    let t31608 = t6889 * t31607;
    let t31609 = t1985 * t31608;
    let t31611 = t214 * t2085;
    (t31594, t31595, t31607, t31608, t31609, t31611)
}
