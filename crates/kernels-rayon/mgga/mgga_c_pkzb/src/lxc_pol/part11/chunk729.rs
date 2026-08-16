//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 729/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk729(t1485: f64, t178: f64, t301: f64, t299: f64, t2003: f64, t53: f64, t2002: f64, t208: f64) -> (f64, f64, f64, f64) {
    let t5612 = t178 * t1485 * t301;
    let t5614 = 0.63517063878621832551e-4_f64 * t299 * t5612;
    let t5627 = t53 * t2003;
    let t5633 = 1.0_f64 / t2002 / t208;
    (t5612, t5614, t5627, t5633)
}
