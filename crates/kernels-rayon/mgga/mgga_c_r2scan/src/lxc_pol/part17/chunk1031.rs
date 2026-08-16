//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1031/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1031(t12960: f64, t11380: f64, t11454: f64, t12437: f64, t12440: f64, t12443: f64, t12944: f64, t12947: f64, t12949: f64, t12953: f64, t12958: f64, t11357: f64, t11566: f64, t11570: f64, t11580: f64, t12721: f64, t12723: f64, t12726: f64, t12728: f64, t12730: f64, t12733: f64, t12939: f64, t12942: f64) -> (f64, f64) {
    let t12961 = t12960 / 2.0_f64;
    let t12962 = t11380 + t12944 + t12947 - t12949 + 0.30487649791575028312e-3_f64 * t12437 + t12953 - t11454 - 0.72042316457491791901e-3_f64 * t12440 - 0.1440846329149835838e-2_f64 * t12443 - t12958 + t12961;
    let t12964 = t12721 - 0.60975299583150056624e-3_f64 * t11566 + 0.86737941314158990616e-4_f64 * t11570 - t12723 - t12726 - t12728 - t12730 - t12733 - t11357 + 0.3842256877732895568e-2_f64 * t11580 + t12939 + t12942 + t12962;
    (t12961, t12964)
}
