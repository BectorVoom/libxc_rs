//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 393/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk393<F: Float>(t28: F, t813: F, t14: F, t829: F, t830: F, t181: F) -> (F, F, F, F, F, F) {
    let t2854 = t813 * t28;
    let t2855 = 1.0 / t2854;
    let t2856 = t14 * t2855;
    let t2857 = t829 * t829;
    let t2858 = t2857 * t830;
    let t2860 = 2.0 * t2856 * t2858;
    let t2861 = 1.0 / t181;
    (t2855, t2856, t2857, t2858, t2860, t2861)
}
