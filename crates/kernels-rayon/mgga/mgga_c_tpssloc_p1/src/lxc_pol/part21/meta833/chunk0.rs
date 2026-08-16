//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2941/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2941(t13784: f64, t17178: f64, t2986: f64, t10189: f64, t5836: f64, t2990: f64, t17161: f64, t17152: f64, t48213: f64, t17863: f64, t42837: f64, t10186: f64, t17808: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t61245 = t2986 * t13784 * t17178;
    let t61250 = t10189 * t5836;
    let t61252 = t2986 * t61250 * t2990;
    let t61258 = t2986 * t13784 * t17161;
    let t61261 = t2986 * t48213 * t17152;
    let t61264 = t2986 * t42837 * t17863;
    let t61273 = t10186 * t17808;
    (t61245, t61252, t61258, t61261, t61264, t61273)
}
