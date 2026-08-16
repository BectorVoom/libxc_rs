//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1060/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1060(t31181: f64, t539: f64, t225: f64, t8471: f64, t6883: f64, t8480: f64, t2006: f64, t552: f64, t1307: f64, t6637: f64, t6888: f64, t794: f64, t8479: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31182 = t539 * t31181;
    let t31189 = t8471 * t225;
    let t31192 = 0.38381794893125283518e-1_f64 * t6883 * t8480;
    let t31193 = t552 * t2006;
    let t31194 = t31193 * t1307;
    let t31195 = t6637 * t31194;
    let t31197 = 0.3289868133696452873e-1_f64 * t6888 * t31195;
    let t31198 = t794 * t8479;
    (t31182, t31189, t31192, t31193, t31194, t31195, t31197, t31198)
}
