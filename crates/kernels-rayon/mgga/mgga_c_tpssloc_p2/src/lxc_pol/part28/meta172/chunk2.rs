//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 840/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk840(t1340: f64, t3777: f64, t1333: f64, t1358: f64, t1362: f64, t1337: f64, t551: f64) -> (f64, f64, f64, f64) {
    let t3778 = t3777 * t1340;
    let t3781 = t1333 * t1358;
    let t3783 = t3777 * t1362;
    let t3787 = 1.0_f64 / t1337 / t551;
    (t3778, t3781, t3783, t3787)
}
