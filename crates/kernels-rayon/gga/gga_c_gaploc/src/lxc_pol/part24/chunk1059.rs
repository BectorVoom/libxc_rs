//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1059/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1059(t16922: f64, t278: f64, t481: f64, t16889: f64, t2547: f64, t686: f64, t1710: f64, t935: f64, t7290: f64, t296: f64, t7112: f64, t830: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21571 = t481 * t16922 * t278;
    let t21636 = t481 * t16889 * t278;
    let t21665 = t481 * t2547 * t686;
    let t21783 = t935 * t1710;
    let t21784 = t7290 * t21783;
    let t21794 = t296 * t7112;
    let t21888 = t830 * t935;
    (t21571, t21636, t21665, t21783, t21784, t21794, t21888)
}
