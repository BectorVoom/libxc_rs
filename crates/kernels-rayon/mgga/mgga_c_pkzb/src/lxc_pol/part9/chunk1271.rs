//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1271/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1271(t22303: f64, t841: f64, t834: f64, t18451: f64, t18454: f64, t18457: f64, t22230: f64, t22234: f64, t22236: f64, t22262: f64, t22265: f64, t22269: f64, t22273: f64, t22277: f64, t22281: f64, t22284: f64, t22287: f64, t22290: f64, t22294: f64, t22297: f64) -> (f64, f64, f64) {
    let t22304 = t841 * t22303;
    let t22306 = t834 * t22303;
    let t22308 = 0.82785e0_f64 * t18451 - 0.49671e0_f64 * t18454 - 0.16557e0_f64 * t18457 - 0.93932222222222222223e0_f64 * t22230 + t22234 - 0.905775e0_f64 * t22236 + 0.905775e0_f64 * t22262 - 0.49671e0_f64 * t22265 + 0.248355e0_f64 * t22269 + 0.745065e0_f64 * t22273 + 0.745065e0_f64 * t22277 + 0.248355e0_f64 * t22281 - 0.49671e0_f64 * t22284 - 0.99342e0_f64 * t22287 - 0.73586666666666666667e0_f64 * t22290 + t22294 + t22297 + 0.16504875e0_f64 * t22304 + 0.258925e1_f64 * t22306;
    (t22304, t22306, t22308)
}
