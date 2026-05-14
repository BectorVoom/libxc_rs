//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 438/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk438<F: Float>(t2819: F, t567: F, t172: F, t32: F, t5: F, t142: F, t814: F, t298: F, t831: F, t28: F, t813: F, t14: F, t829: F, t830: F, t181: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2820 = t567 * t2819;
    let t2849 = 0.14764770444444444444e-2 * t5 * t172 * t32;
    let t2850 = t142 * t814;
    let t2853 = 0.35616666666666666667e-1 * t298 * t2850 * t831;
    let t2854 = t813 * t28;
    let t2855 = 1.0 / t2854;
    let t2856 = t14 * t2855;
    let t2857 = t829 * t829;
    let t2858 = t2857 * t830;
    let t2860 = 2.0 * t2856 * t2858;
    let t2861 = 1.0 / t181;
    (t2820, t2849, t2850, t2853, t2855, t2856, t2857, t2858, t2860, t2861)
}
