//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 205/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk205<F: Float>(t829: F, t830: F, t815: F, t2: F, t45: F, t142: F, t56: F, t69: F, t47: F, t52: F) -> (F, F, F, F, F, F, F) {
    let t831 = t829 * t830;
    let t833 = 1.0 * t815 * t831;
    let t834 = t45 * t2;
    let t836 = t69 * t142 * t56;
    let t839 = t45 * t47;
    let t840 = t52 * t52;
    let t841 = 1.0 / t840;
    (t831, t833, t834, t836, t839, t840, t841)
}
