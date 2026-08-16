//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1309/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1309(t4083: f64, t8669: f64, t4110: f64, t8589: f64, t829: f64, t830: f64, t52991: f64, t53011: f64, t53015: f64, t14182: f64, t26958: f64, t11375: f64, t14193: f64, t14918: f64, t22142: f64, t22343: f64, t2384: f64, t52179: f64, t52381: f64, t52994: f64, t52997: f64, t53009: f64, t53019: f64, t827: f64, t8793: f64) -> f64 {
    let t54915 = 7.0_f64 / 144.0_f64 * t8669 * t4083;
    let t54916 = t8589 * t4110;
    let t54918 = t829 * t830 * t54916;
    let t54923 = 7.0_f64 / 72.0_f64 * t52991;
    let t54927 = 7.0_f64 / 1152.0_f64 * t53011;
    let t54928 = 35.0_f64 / 216.0_f64 * t53015;
    let t54937 = 7.0_f64 / 72.0_f64 * t26958 * t14182;
    let t54940 = t54915 - t827 * t54918 / 48.0_f64 - t2384 * t14918 / 96.0_f64 + t54923 - t52994 / 12.0_f64 - t52997 / 12.0_f64 - t53009 / 768.0_f64 - t54927 + t54928 + t53019 / 768.0_f64 - t22142 * t4083 / 96.0_f64 + t22343 * t14193 / 48.0_f64 - t8793 * t52381 / 16.0_f64 - t54937 - t11375 * t52179 / 48.0_f64;
    t54940
}
