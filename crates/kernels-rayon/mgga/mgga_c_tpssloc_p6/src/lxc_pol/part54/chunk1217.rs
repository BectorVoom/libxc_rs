//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1217/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1217(t1378: f64, t33293: f64, t6906: f64, t7936: f64, t6889: f64, t1985: f64, t2015: f64, t3887: f64, t31611: f64, t7691: f64, t6888: f64, t7700: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33294 = t1378 * t33293;
    let t33296 = t6906 * t7936;
    let t33297 = t6889 * t33296;
    let t33298 = t1985 * t33297;
    let t33300 = t7936 * t2015;
    let t33301 = t3887 * t33300;
    let t33307 = t31611 * t7691;
    let t33308 = t6888 * t33307;
    let t33310 = t31611 * t7700;
    (t33294, t33296, t33297, t33298, t33301, t33307, t33308, t33310)
}
