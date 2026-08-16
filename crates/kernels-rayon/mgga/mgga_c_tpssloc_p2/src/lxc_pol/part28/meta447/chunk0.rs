//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1633/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1633(t23917: f64, t510: f64, t1266: f64, t7056: f64, t671: f64, t7156: f64, t111: f64, t7039: f64) -> (f64, f64, f64, f64) {
    let t23918 = t510 * t23917;
    let t23929 = t1266 * t7056;
    let t23933 = t7156 * t671;
    let t23938 = t7039 * t111;
    (t23918, t23929, t23933, t23938)
}
