//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1099/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1099<F: Float>(t358: F, t363: F, t428: F, t1624: F, t92356: F, t53: F, t22582: F, t92353: F, t401: F, t72: F, t92818: F, t1669: F, t22512: F, t22547: F, t25752: F, t69: F, t9: F) -> (F, F, F, F, F, F, F, F, F) {
    let t93124 = t428 * t358 * t363;
    let t93129 = t1624 * t92356;
    let t93131 = t53 * t358 * t363;
    let t93136 = t92353 * t22582;
    let t93138 = t401 * t358 * t363;
    let t93143 = t92818 * t72;
    let t93157 = t1669 * t22512;
    let t93163 = t22547 * t25752;
    let t93164 = t9 * t69;
    (t93124, t93129, t93131, t93136, t93138, t93143, t93157, t93163, t93164)
}
