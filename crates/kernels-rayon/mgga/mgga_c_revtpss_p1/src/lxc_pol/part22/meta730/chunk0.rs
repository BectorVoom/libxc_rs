//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2787/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2787(t820: f64, t823: f64, t9948: f64, t839: f64, t10841: f64, t10845: f64, t10815: f64, t2648: f64, t2756: f64, t2681: f64, t2719: f64, t2726: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40360 = t820 * t823 * t9948;
    let t40361 = t40360 * t839;
    let t40374 = t10845 * t10841;
    let t40393 = t10815 * t2648;
    let t40395 = t10815 * t2756;
    let t40398 = t820 * t2719 * t2681;
    let t40399 = t40398 * t2726;
    (t40360, t40361, t40374, t40393, t40395, t40398, t40399)
}
