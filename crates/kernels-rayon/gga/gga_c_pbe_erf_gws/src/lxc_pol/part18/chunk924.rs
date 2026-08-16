//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 924/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk924(t10335: f64, t1815: f64, t639: f64, t1044: f64, t2705: f64, t7199: f64, t3469: f64, t617: f64, t1809: f64, t1620: f64, t661: f64, t10307: f64, t10309: f64, t10311: f64, t10317: f64, t10319: f64, t10321: f64, t10322: f64, t10324: f64, t10328: f64, t10330: f64, t10332: f64, t10334: f64, t7147: f64) -> (f64, f64, f64, f64, f64) {
    let t10336 = t1815 * t10335;
    let t10338 = 8.0_f64 / 45.0_f64 * t639 * t10336;
    let t10339 = t2705 * t1044;
    let t10340 = t7199 * t10339;
    let t10342 = 16.0_f64 / 45.0_f64 * t639 * t10340;
    let t10343 = t3469 * t617;
    let t10344 = t1809 * t10343;
    let t10346 = 16.0_f64 / 45.0_f64 * t1620 * t10344;
    let t10347 = t3469 * t661;
    let t10348 = t1815 * t10347;
    let t10350 = 8.0_f64 / 45.0_f64 * t639 * t10348;
    let t10351 = t10307 + t10309 + t10311 + t10317 - t10319 + t10321 + t10322 + t7147 + t10324 + t10328 + t10330 + t10332 - t10334 - t10338 + t10342 - t10346 + t10350;
    (t10338, t10342, t10346, t10350, t10351)
}
