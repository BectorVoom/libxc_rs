//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 924/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk924<F: Float>(t10335: F, t1815: F, t639: F, t1044: F, t2705: F, t7199: F, t3469: F, t617: F, t1809: F, t1620: F, t661: F, t10307: F, t10309: F, t10311: F, t10317: F, t10319: F, t10321: F, t10322: F, t10324: F, t10328: F, t10330: F, t10332: F, t10334: F, t7147: F) -> (F, F, F, F, F) {
    let t10336 = t1815 * t10335;
    let t10338 = F::new(8.0) / F::new(45.0) * t639 * t10336;
    let t10339 = t2705 * t1044;
    let t10340 = t7199 * t10339;
    let t10342 = F::new(16.0) / F::new(45.0) * t639 * t10340;
    let t10343 = t3469 * t617;
    let t10344 = t1809 * t10343;
    let t10346 = F::new(16.0) / F::new(45.0) * t1620 * t10344;
    let t10347 = t3469 * t661;
    let t10348 = t1815 * t10347;
    let t10350 = F::new(8.0) / F::new(45.0) * t639 * t10348;
    let t10351 = t10307 + t10309 + t10311 + t10317 - t10319 + t10321 + t10322 + t7147 + t10324 + t10328 + t10330 + t10332 - t10334 - t10338 + t10342 - t10346 + t10350;
    (t10338, t10342, t10346, t10350, t10351)
}
