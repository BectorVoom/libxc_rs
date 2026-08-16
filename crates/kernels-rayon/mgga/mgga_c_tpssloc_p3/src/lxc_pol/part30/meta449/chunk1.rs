//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1710/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1710(t242: f64, t6943: f64, t1336: f64) -> (f64, f64) {
    let t22832 = t6943 * t242;
    let t22833 = t1336 * t22832;
    (t22832, t22833)
}
