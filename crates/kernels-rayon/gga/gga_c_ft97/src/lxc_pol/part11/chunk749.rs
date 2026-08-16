//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 749/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk749(t10121: f64, t762: f64, t242: f64, t1882: f64, t2591: f64, t2596: f64, t265: f64, t724: f64, t9596: f64, t726: f64, t8232: f64, t2619: f64, t684: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10122 = t762 * t10121;
    let t10123 = t242 * t10122;
    let t10126 = t1882 * t2591;
    let t10128 = t1882 * t2596;
    let t10131 = t724 * t265 * t9596;
    let t10134 = t8232 * t726;
    let t10137 = t724 * t2619 * t684;
    (t10122, t10123, t10126, t10128, t10131, t10134, t10137)
}
