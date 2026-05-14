//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 582/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk582<F: Float>(t1419: F, t5804: F, t1409: F, t5526: F, t1650: F, t532: F, t4061: F, t1444: F, t822: F, t1976: F, t743: F, t1968: F, t733: F, t1971: F, t738: F, t167: F, t4023: F, t4047: F, t4050: F, t4053: F, t4059: F, t4081: F, t4089: F, t4093: F, t5654: F) -> (F, F, F, F) {
    let t5805 = t5804 * t1419;
    let t5808 = t1409 * t5526;
    let t5814 = t532 * t1650;
    let t5816 = t4061 * t1650;
    let t5820 = t822 * t1444;
    let t5829 = t743 * t1976;
    let t5831 = t733 * t1968;
    let t5833 = t738 * t1971;
    let t5835 = -0.11955719325063177623e-1 * t5814 + 0.10359077815592613752e-3 * t5816 - 0.23911438650126355246e-1 * t4059 * t167 + 0.10359077815592613752e-3 * t5820 * t167 - 0.23911438650126355246e-1 * t4023 * t5654 + 0.15538616723388920628e-3 * t4093 * t5654 + 0.4684e-2 * t4081 - 0.13208333333333333333e-2 * t4089 + t4047 - t4050 - t4053 - 0.117630625e-4 * t5829 + 0.4684e-2 * t5831 - 0.13208333333333333333e-2 * t5833;
    (t5805, t5808, t5820, t5835)
}
