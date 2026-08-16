//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 822/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk822(t56: f64, t931: f64, t19: f64, t274: f64, t6161: f64, t2132: f64, t328: f64, t824: f64, t822: f64, t6277: f64, t858: f64, t2407: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6658 = t56 * t931;
    let t6659 = t6658 * t19;
    let t6665 = t274 * t6161;
    let t6670 = t2132 * t328;
    let t6671 = t824 * t6670;
    let t6672 = t822 * t6671;
    let t6673 = t858 * t6277;
    let t6674 = t2407 * t6673;
    (t6659, t6665, t6670, t6671, t6672, t6674)
}
