//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2061/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2061(t41362: f64, t831: f64, t2628: f64, t2690: f64, t812: f64, t835: f64, t9972: f64, t2617: f64, t9666: f64, t776: f64, t9975: f64, t6589: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41363 = t41362 * t831;
    let t41385 = t812 * t2628 * t2690;
    let t41414 = t812 * t9972 * t835;
    let t41424 = t2617 * t9666;
    let t41453 = t9975 * t776;
    let t41466 = t6589 * t67;
    (t41363, t41385, t41414, t41424, t41453, t41466)
}
