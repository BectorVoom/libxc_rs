//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1393/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1393(t114696: f64, t1484: f64, t6552: f64, t6637: f64, t31376: f64, t4119: f64, t23110: f64, t23185: f64, t33379: f64, t1888: f64, t232: f64, t6646: f64, t92745: f64) -> (f64, f64, f64, f64) {
    let t121517 = t6552 * t6637 * t114696 * t1484;
    let t121521 = t6552 * t6637 * t31376 * t4119;
    let t121524 = t23185 * t23110 * t33379;
    let t121528 = t1888 * t6646 * t92745 * t232;
    (t121517, t121521, t121524, t121528)
}
