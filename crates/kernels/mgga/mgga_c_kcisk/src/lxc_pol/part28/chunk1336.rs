//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1336/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1336<F: Float>(t34415: F, t9720: F, t33234: F, t9991: F, t33162: F, t34484: F, t9721: F, t34594: F, t34579: F, t9724: F, t112761: F, t34547: F, t33196: F, t33218: F, t964: F, t1310: F, t2021: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t117791 = t9720 * t34415;
    let t117808 = 0.34722222222222222222e-2 * t9991 * t33234;
    let t117810 = 0.34722222222222222222e-2 * t9991 * t33162;
    let t117812 = 0.34722222222222222222e-2 * t9721 * t34484;
    let t117814 = 0.13402777777777777778e-2 * t34594 * t33162;
    let t117824 = t9724 * t34579;
    let t117840 = t112761 * t34547;
    let t117841 = t33196 * t117840;
    let t117857 = t964 * t33218;
    let t117866 = t1310 * t2021;
    (t117791, t117808, t117810, t117812, t117814, t117824, t117840, t117841, t117857, t117866)
}
