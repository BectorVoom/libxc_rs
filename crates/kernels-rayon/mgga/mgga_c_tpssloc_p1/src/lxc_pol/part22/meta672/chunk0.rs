//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2227/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2227(t17152: f64, t2986: f64, t48213: f64, t17863: f64, t42837: f64, t10186: f64, t17808: f64, t10236: f64, t17635: f64, t13835: f64, t13847: f64, t13839: f64, t48279: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t61261 = t2986 * t48213 * t17152;
    let t61264 = t2986 * t42837 * t17863;
    let t61273 = t10186 * t17808;
    let t61279 = t10236 * t17635;
    let t61288 = t2986 * t13847 * t13835;
    let t61291 = t2986 * t48279 * t13839;
    (t61261, t61264, t61273, t61279, t61288, t61291)
}
