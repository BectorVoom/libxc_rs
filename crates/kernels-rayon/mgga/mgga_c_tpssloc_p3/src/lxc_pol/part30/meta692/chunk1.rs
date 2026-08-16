//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2205/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2205(t1530: f64, t584: f64, t86730: f64, t25372: f64, t5397: f64, t868: f64, t28248: f64, t81547: f64, t5660: f64, t606: f64, t17109: f64, t25: f64) -> (f64, f64, f64, f64, f64) {
    let t98069 = t86730 * t584 * t1530;
    let t98071 = 2.0_f64 * t25372 * t98069;
    let t98075 = t5397 * t868;
    let t98079 = t81547 * t28248;
    let t98082 = t606 * t5660;
    let t98086 = t25 * t17109;
    (t98071, t98075, t98079, t98082, t98086)
}
