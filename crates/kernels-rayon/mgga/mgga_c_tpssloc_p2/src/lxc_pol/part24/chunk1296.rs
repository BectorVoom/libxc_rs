//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1296/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1296(t22480: f64, t4034: f64, t22574: f64, t55246: f64, t8643: f64, t23858: f64, t6876: f64, t12492: f64, t12507: f64, t1266: f64, t1980: f64, t22600: f64, t2364: f64, t26103: f64, t6517: f64, t80609: f64, t80611: f64, t80614: f64, t80617: f64, t80620: f64, t80622: f64, t80625: f64, t80627: f64, t80629: f64, t80633: f64, t80635: f64, t80637: f64, t81410: f64) -> f64 {
    let t81412 = 6.0_f64 * t4034 * t22480;
    let t81419 = 9.0_f64 * t22574 * t8643 * t55246;
    let t81422 = 6.0_f64 * t6876 * t23858;
    let t81423 = t12492 * t1980 - 6.0_f64 * t12507 * t6517 - 6.0_f64 * t1266 * t22600 - 6.0_f64 * t2364 * t26103 + t80609 - t80611 + t80614 - t80617 - t80620 - t80622 - t80625 - t80627 - t80629 + t80633 + t80635 + t80637 + t81410 - t81412 - t81419 + t81422;
    t81423
}
