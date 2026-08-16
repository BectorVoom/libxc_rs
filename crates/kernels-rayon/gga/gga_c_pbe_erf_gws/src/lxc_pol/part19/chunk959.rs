//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 959/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk959(t10782: f64, t10832: f64, t598: f64, t186: f64, t185: f64, t5355: f64, t3488: f64, t612: f64, t3584: f64, t723: f64, t3398: f64, t586: f64) -> (f64, f64, f64, f64, f64) {
    let t10833 = t10782 + t10832;
    let t10834 = t598 * t10833;
    let t10835 = t186 * t10834;
    let t10837 = 2.0_f64 / 15.0_f64 * t185 * t10835;
    let t10838 = 4.0_f64 / 135.0_f64 * t5355;
    let t10840 = 2.0_f64 / 15.0_f64 * t3488 * t612;
    let t10841 = t3584 * t723;
    let t10843 = t3398 * t586;
    (t10837, t10838, t10840, t10841, t10843)
}
