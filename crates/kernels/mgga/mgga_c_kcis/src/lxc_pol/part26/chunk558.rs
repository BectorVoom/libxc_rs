//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 558/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk558<F: Float>(t1976: F, t743: F, t1968: F, t733: F, t1971: F, t738: F, t167: F, t4023: F, t4047: F, t4050: F, t4053: F, t4059: F, t4081: F, t4089: F, t4093: F, t5654: F, t5814: F, t5816: F, t5820: F) -> (F, F, F, F) {
    let t5829 = t743 * t1976;
    let t5831 = t733 * t1968;
    let t5833 = t738 * t1971;
    let t5835 = -F::new(0.11955719325063177623e-1) * t5814 + F::new(0.10359077815592613752e-3) * t5816 - F::new(0.23911438650126355246e-1) * t4059 * t167 + F::new(0.10359077815592613752e-3) * t5820 * t167 - F::new(0.23911438650126355246e-1) * t4023 * t5654 + F::new(0.15538616723388920628e-3) * t4093 * t5654 + F::new(0.4684e-2) * t4081 - F::new(0.13208333333333333333e-2) * t4089 + t4047 - t4050 - t4053 - F::new(0.117630625e-4) * t5829 + F::new(0.4684e-2) * t5831 - F::new(0.13208333333333333333e-2) * t5833;
    (t5829, t5831, t5833, t5835)
}
