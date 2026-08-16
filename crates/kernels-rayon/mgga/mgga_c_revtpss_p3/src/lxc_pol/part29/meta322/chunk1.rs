//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1230/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1230(t10841: f64, t2484: f64, t2482: f64, t596: f64, t823: f64, t2487: f64, t27: f64, t2719: f64, t221: f64, t2485: f64, t2724: f64, t2741: f64, t2756: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10842 = t2484 * t10841;
    let t10845 = t2482 * t823 * t596;
    let t10846 = t10845 * t2487;
    let t10850 = t2482 * t2719 * t27;
    let t10852 = t2485 * t221 * t2724;
    let t10853 = t10850 * t10852;
    let t10855 = t2741 * t2756;
    (t10842, t10845, t10846, t10852, t10853, t10855)
}
