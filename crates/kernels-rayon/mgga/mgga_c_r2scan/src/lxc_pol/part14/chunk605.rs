//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 605/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk605(t1060: f64, t3336: f64, t783: f64, t1779: f64, t9: f64, t2096: f64, t2105: f64, t265: f64) -> (f64, f64, f64, f64) {
    let t3338 = t783 * t3336 * t1060;
    let t3341 = 1.0_f64 / t9 / t1779;
    let t3342 = t2096 * t3341;
    let t3344 = t3342 * t265 * t2105;
    (t3338, t3341, t3342, t3344)
}
