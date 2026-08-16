//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 659/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk659(t10734: f64, t2958: f64, t7068: f64, t2580: f64, t1897: f64, t2549: f64, t3444: f64, t8528: f64, t883: f64, t2562: f64, t943: f64, t3437: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10735 = 0.42725145723012357132e-3_f64 * t10734;
    let t10736 = t2958 * t7068;
    let t10737 = t2580 * t10736;
    let t10739 = 0.15381052460284448567e-1_f64 * t1897 * t10737;
    let t10740 = t2549 * t3444;
    let t10741 = 0.32043859292259267849e-3_f64 * t10740;
    let t10742 = t883 * t8528;
    let t10743 = t2562 * t10742;
    let t10744 = t943 * t10743;
    let t10745 = 0.32043859292259267849e-3_f64 * t10744;
    let t10746 = t2549 * t3437;
    (t10735, t10736, t10739, t10741, t10745, t10746)
}
