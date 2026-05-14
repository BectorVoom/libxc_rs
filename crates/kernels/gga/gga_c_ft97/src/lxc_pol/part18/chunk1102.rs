//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1102/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1102<F: Float>(t22845: F, t5607: F, t1300: F, t22855: F, t5603: F, t22598: F, t22613: F, t415: F, t172: F, t5589: F, t72: F, t22796: F, t22799: F, t1602: F, t92685: F, t1681: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t93188 = t22845 * t5607;
    let t93189 = t1300 * t93188;
    let t93191 = t5603 * t22855;
    let t93192 = t1300 * t93191;
    let t93195 = t22613 * t415 * t22598;
    let t93252 = t5589 * t172;
    let t93253 = t93252 * t72;
    let t93255 = t22796 * t93253 * t22799;
    let t93268 = t1602 * t92685;
    let t93271 = sigma0 * t1681;
    (t93188, t93189, t93191, t93192, t93195, t93252, t93253, t93255, t93268, t93271)
}
