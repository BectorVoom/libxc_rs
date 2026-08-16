//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1372/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1372(t1484: f64, t6552: f64, t6637: f64, t98598: f64, t25319: f64, t5544: f64, t23035: f64, t5527: f64, t1888: f64, t21025: f64, t22996: f64, t22986: f64, t25249: f64, t5617: f64, t6646: f64) -> (f64, f64, f64, f64, f64) {
    let t105574 = t6552 * t6637 * t98598 * t1484;
    let t105578 = t6552 * t6637 * t25319 * t5544;
    let t105582 = t23035 * t6637 * t25319 * t5527;
    let t105586 = t1888 * t22996 * t21025;
    let t105596 = t22986 * t6646 * t25249 * t5617;
    (t105574, t105578, t105582, t105586, t105596)
}
