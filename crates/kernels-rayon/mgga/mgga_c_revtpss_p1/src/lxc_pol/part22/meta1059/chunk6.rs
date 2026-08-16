//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3769/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3769(t1222: f64, t20293: f64, t57484: f64, t17735: f64, t70646: f64, t17423: f64, t21014: f64, t17708: f64, t59498: f64, t1042: f64, t17505: f64, t17584: f64, t17589: f64, t17739: f64, t17750: f64, t17800: f64, t20795: f64, t21093: f64, t3368: f64, t3372: f64, t3720: f64, t44551: f64, t5384: f64, t58803: f64, t59379: f64, t59386: f64, t59391: f64, t71440: f64) -> f64 {
    let t72000 = t1222 * t57484 * t20293;
    let t72002 = t17735 * t70646;
    let t72005 = t21014 * t17423;
    let t72011 = t59498 * t17708;
    let t72014 = -0.28582678745379824648e-3_f64 * t5384 * t1042 * t21093 * t3372 + 0.28582678745379824648e-3_f64 * t59379 - 0.57165357490759649296e-3_f64 * t5384 * t1042 * t21093 * t3368 - 0.15244095330869239812e-2_f64 * t17505 * t17584 - 0.30488190661738479624e-2_f64 * t17505 * t17589 + 0.19055119163586549765e-3_f64 * t59386 + 0.60976381323476959248e-2_f64 * t71440 * t17800 - 0.7622047665434619906e-3_f64 * t59391 - 7.0_f64 / 972.0_f64 * t72000 + 0.60976381323476959248e-2_f64 * t72002 * t17739 - 0.60976381323476959248e-2_f64 * t72005 + 0.85748036236139473944e-3_f64 * t44551 * t3720 * t20795 * t58803 - 0.25724410870841842183e-2_f64 * t72011 * t17750;
    t72014
}
