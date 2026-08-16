//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1234/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1234(t1112: f64, t361: f64, t51020: f64, t3209: f64, t51682: f64, t3958: f64, t6148: f64, t352: f64, t830: f64, t1178: f64, t8713: f64, t2299: f64, t371: f64, t3970: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53799 = t361 * t51020 * t1112;
    let t53806 = t51682 * t3209;
    let t53840 = t3958 * t6148;
    let t53841 = t830 * t352;
    let t53860 = t1178 * t8713;
    let t53865 = t3970 * t2299 * t371;
    (t53799, t53806, t53840, t53841, t53860, t53865)
}
