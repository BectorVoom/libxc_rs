//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1697/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1697(t16355: f64, t210: f64, t12308: f64, t12310: f64, t12317: f64, t12323: f64, t12325: f64, t12330: f64, t12336: f64, t1315: f64, t1363: f64, t1369: f64, t16321: f64, t16325: f64, t16331: f64, t16333: f64, t16338: f64, t16341: f64, t16346: f64, t16347: f64, t16350: f64, t16354: f64, t1831: f64, t3783: f64, t3876: f64, t5240: f64, t5314: f64, t559: f64) -> (f64, f64) {
    let t16356 = t210 * t16355;
    let t16361 = -t16321 * t1369 / 384.0_f64 + t16325 - t12336 * t1831 / 768.0_f64 - t3783 * t5314 / 384.0_f64 + t16331 - t1363 * t16333 / 768.0_f64 + t16338 - t5240 * t3876 / 768.0_f64 - 35.0_f64 / 216.0_f64 * t16341 - 35.0_f64 / 108.0_f64 * t12308 + 7.0_f64 / 144.0_f64 * t12310 - t16346 + t16347 * t559 / 3072.0_f64 + 119.0_f64 / 13824.0_f64 * t16350 - 7.0_f64 / 48.0_f64 * t12317 + t16354 - t1315 * t16356 / 48.0_f64 - 7.0_f64 / 4608.0_f64 * t12323 + 119.0_f64 / 6912.0_f64 * t12325 - t12330;
    (t16356, t16361)
}
