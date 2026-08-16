//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1337/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1337(t112: f64, t24954: f64, t111: f64, t24542: f64, t2312: f64, t2314: f64, t2363: f64, t2364: f64, t24552: f64, t24932: f64, t4034: f64, t652: f64, t672: f64, t7408: f64, t80609: f64, t80611: f64, t80614: f64, t80617: f64, t80620: f64, t80622: f64, t80625: f64, t80627: f64, t80629: f64, t80633: f64, t80635: f64, t80637: f64, t81410: f64, t81412: f64) -> (f64, f64, f64) {
    let t85423 = t24954 * t112;
    let t85428 = t24542 * t111;
    let t85442 = -6.0_f64 * t2363 * t652 * t7408 - 3.0_f64 * t2312 * t7408 - 6.0_f64 * t2314 * t24552 - 6.0_f64 * t2364 * t24932 - 6.0_f64 * t24552 * t4034 - 6.0_f64 * t672 * t85428 + t80609 - t80611 + t80614 - t80617 - t80620 - t80622 - t80625 - t80627 - t80629 + t80633 + t80635 + t80637 + t81410 - t81412;
    (t85423, t85428, t85442)
}
