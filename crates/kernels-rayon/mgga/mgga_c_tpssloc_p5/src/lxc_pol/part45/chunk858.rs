//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 858/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk858(t1985: f64, t31120: f64, t6906: f64, t6992: f64, t6889: f64, t22674: f64, t8458: f64, t6897: f64, t2006: f64, t214: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31122 = 0.16449340668482264365e-1_f64 * t1985 * t31120;
    let t31123 = t6906 * t6992;
    let t31124 = t6889 * t31123;
    let t31126 = 0.16449340668482264365e-1_f64 * t1985 * t31124;
    let t31127 = t22674 * t8458;
    let t31129 = 0.82246703342411321825e-2_f64 * t6897 * t31127;
    let t31137 = t214 * t2006;
    (t31122, t31123, t31124, t31126, t31127, t31129, t31137)
}
