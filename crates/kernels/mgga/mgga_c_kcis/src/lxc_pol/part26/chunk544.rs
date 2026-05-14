//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 544/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk544<F: Float>(t1437: F, t5477: F, t1451: F, t1430: F, t5441: F, t5427: F, t542: F, t1330: F, t104: F, t111: F, t120: F, t4054: F, t4055: F, t4060: F, t4062: F, t4073: F, t4858: F, t4865: F, t4881: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5836 = t1437 * t5477;
    let t5839 = t1451 * t5477;
    let t5842 = t1430 * t5477;
    let t5845 = t1451 * t5441;
    let t5848 = t1430 * t5427;
    let t5851 = t1430 * t5441;
    let t5854 = t542 * t5427;
    let t5857 = t1437 * t5441;
    let t5860 = t1330 * t5427;
    let t5866 = -0.1585e-2 * t4865 * t5836 - 0.10082625e-4 * t4881 * t5839 + 0.7026e-2 * t4858 * t5842 - 0.10082625e-4 * t120 * t5845 - 0.672175e-5 * t120 * t5848 + 0.7026e-2 * t104 * t5851 + 0.1171e-2 * t104 * t5854 - 0.1585e-2 * t111 * t5857 - 0.52833333333333333333e-3 * t111 * t5860 - 0.117630625e-4 * t4073 - 0.11955719325063177623e-1 * t4055 + 0.10359077815592613752e-3 * t4062 - t4054 + t4060;
    (t5836, t5839, t5842, t5845, t5848, t5851, t5854, t5857, t5860, t5866)
}
