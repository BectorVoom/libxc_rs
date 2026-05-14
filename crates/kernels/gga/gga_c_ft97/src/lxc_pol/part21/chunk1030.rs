//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1030/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1030<F: Float>(t22582: F, t8042: F, t1624: F, t92356: F, t92353: F, t22533: F, t7837: F, t69: F, t9: F, t5612: F, t1669: F, t22521: F, t2258: F, t2035: F, t5551: F, t22855: F, t5603: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t93122 = t8042 * t22582;
    let t93129 = t1624 * t92356;
    let t93136 = t92353 * t22582;
    let t93153 = t7837 * t22533;
    let t93164 = t9 * t69;
    let t93165 = t93164 * t5612;
    let t93168 = t1669 * t22521;
    let t93169 = t69 * t2258;
    let t93178 = t2035 * t5551;
    let t93191 = t5603 * t22855;
    (t93122, t93129, t93136, t93153, t93164, t93165, t93168, t93169, t93178, t93191)
}
