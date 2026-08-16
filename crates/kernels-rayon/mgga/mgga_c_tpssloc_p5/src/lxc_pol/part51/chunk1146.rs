//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1146/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1146(t214: f64, t31108: f64, t1985: f64, t6883: f64, t8455: f64, t8459: f64, t22666: f64, t8458: f64, t6906: f64, t6992: f64, t6889: f64, t22674: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31109 = t214 * t31108;
    let t31111 = 0.16449340668482264365e-1_f64 * t1985 * t31109;
    let t31113 = 0.38381794893125283518e-1_f64 * t6883 * t8455;
    let t31115 = 0.38381794893125283518e-1_f64 * t6883 * t8459;
    let t31120 = t22666 * t8458;
    let t31122 = 0.16449340668482264365e-1_f64 * t1985 * t31120;
    let t31123 = t6906 * t6992;
    let t31124 = t6889 * t31123;
    let t31126 = 0.16449340668482264365e-1_f64 * t1985 * t31124;
    let t31127 = t22674 * t8458;
    (t31109, t31111, t31113, t31115, t31120, t31122, t31123, t31124, t31126, t31127)
}
