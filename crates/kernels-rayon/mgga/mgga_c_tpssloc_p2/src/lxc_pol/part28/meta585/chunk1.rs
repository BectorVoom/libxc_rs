//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1876/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1876(t23035: f64, t2379: f64, t25319: f64, t6637: f64, t1887: f64, t81959: f64, t25248: f64, t25249: f64, t1888: f64, t232: f64, t4265: f64, t6646: f64, t828: f64) -> (f64, f64, f64, f64) {
    let t87640 = t23035 * t6637 * t25319 * t2379;
    let t87642 = t81959 * t1887;
    let t87645 = t87642 * t25248 * t25249 * t2379;
    let t87650 = t1888 * t6646 * t4265 * t828 * t232;
    (t87640, t87642, t87645, t87650)
}
