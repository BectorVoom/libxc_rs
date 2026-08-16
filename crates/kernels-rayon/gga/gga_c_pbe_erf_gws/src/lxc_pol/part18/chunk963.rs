//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 963/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk963(t10784: f64, t7435: f64, t587: f64, t10788: f64, t7699: f64, t3445: f64, t579: f64, t3444: f64, t582: f64, t185: f64, t1006: f64, t2756: f64) -> (f64, f64, f64, f64, f64) {
    let t10861 = t7435 * t10784;
    let t10863 = 32.0_f64 / 81.0_f64 * t587 * t10861;
    let t10864 = t7699 * t10788;
    let t10866 = 16.0_f64 / 27.0_f64 * t587 * t10864;
    let t10870 = 2.0_f64 / 15.0_f64 * t579 * t3445;
    let t10871 = t582 * t3444;
    let t10872 = t185 * t10871;
    let t10873 = 4.0_f64 / 45.0_f64 * t10872;
    let t10874 = t1006 * t2756;
    (t10863, t10866, t10870, t10873, t10874)
}
