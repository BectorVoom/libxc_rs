//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1010/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1010(t1985: f64, t24138: f64, t6889: f64, t6906: f64, t22685: f64, t22686: f64, t31611: f64, t22934: f64, t2085: f64, t3791: f64, t1992: f64, t550: f64, t6976: f64) -> (f64, f64, f64, f64, f64) {
    let t115368 = t1985 * t6889 * t6906 * t24138;
    let t115372 = t22685 * t31611 * t22686;
    let t115378 = t1985 * t31611 * t22934;
    let t115384 = t2085 * t3791;
    let t115387 = t1992 * t6976 * t115384 * t550;
    (t115368, t115372, t115378, t115384, t115387)
}
