//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1249/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1249(t1162: f64, t13796: f64, t2195: f64, t3989: f64, t3307: f64, t875: f64, t14127: f64, t2503: f64, t1118: f64, t13859: f64, t14682: f64, t2158: f64) -> (f64, f64, f64, f64) {
    let t53639 = t3989 * t13796 * t1162 * t2195;
    let t53643 = t3989 * t13796 * t3307 * t875;
    let t53645 = t14127 * t2503;
    let t53664 = t13859 * t14682 * t1118 * t2158;
    (t53639, t53643, t53645, t53664)
}
