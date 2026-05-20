//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3769/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3769<F: Float>(t1222: F, t20293: F, t57484: F, t17735: F, t70646: F, t17423: F, t21014: F, t17708: F, t59498: F, t1042: F, t17505: F, t17584: F, t17589: F, t17739: F, t17750: F, t17800: F, t20795: F, t21093: F, t3368: F, t3372: F, t3720: F, t44551: F, t5384: F, t58803: F, t59379: F, t59386: F, t59391: F, t71440: F) -> F {
    let t72000 = t1222 * t57484 * t20293;
    let t72002 = t17735 * t70646;
    let t72005 = t21014 * t17423;
    let t72011 = t59498 * t17708;
    let t72014 = -F::cast_from(0.28582678745379824648e-3_f64) * t5384 * t1042 * t21093 * t3372 + F::cast_from(0.28582678745379824648e-3_f64) * t59379 - F::cast_from(0.57165357490759649296e-3_f64) * t5384 * t1042 * t21093 * t3368 - F::cast_from(0.15244095330869239812e-2_f64) * t17505 * t17584 - F::cast_from(0.30488190661738479624e-2_f64) * t17505 * t17589 + F::cast_from(0.19055119163586549765e-3_f64) * t59386 + F::cast_from(0.60976381323476959248e-2_f64) * t71440 * t17800 - F::cast_from(0.7622047665434619906e-3_f64) * t59391 - F::new(7.0) / F::new(972.0) * t72000 + F::cast_from(0.60976381323476959248e-2_f64) * t72002 * t17739 - F::cast_from(0.60976381323476959248e-2_f64) * t72005 + F::cast_from(0.85748036236139473944e-3_f64) * t44551 * t3720 * t20795 * t58803 - F::cast_from(0.25724410870841842183e-2_f64) * t72011 * t17750;
    t72014
}
