//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 674/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk674(t1882: f64, t2591: f64, t2596: f64, t726: f64, t8232: f64, t2587: f64, t2614: f64, t2581: f64, t2542: f64, t761: f64, t192: f64, t7514: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10126 = t1882 * t2591;
    let t10128 = t1882 * t2596;
    let t10134 = t8232 * t726;
    let t10140 = t1882 * t2587;
    let t10146 = t1882 * t2614;
    let t10148 = t1882 * t2581;
    let t10153 = t2542 * t761;
    let t10157 = t192 * t7514;
    (t10126, t10128, t10134, t10140, t10146, t10148, t10153, t10157)
}
