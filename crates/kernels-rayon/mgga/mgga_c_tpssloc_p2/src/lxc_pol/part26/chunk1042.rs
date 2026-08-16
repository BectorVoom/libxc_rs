//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1042/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1042(t12303: f64, t3870: f64, t820: f64, t12189: f64, t1329: f64, t3726: f64, t3770: f64, t119: f64, t12012: f64, t210: f64, t12211: f64, t3766: f64) -> (f64, f64, f64, f64, f64) {
    let t12305 = t3870 * t820 * t12303;
    let t12308 = t12189 * t1329;
    let t12310 = t3726 * t3770;
    let t12313 = t210 * t119 * t12012;
    let t12317 = t12211 * t3766;
    (t12305, t12308, t12310, t12313, t12317)
}
