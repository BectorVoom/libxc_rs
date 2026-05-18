//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 943/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk943<F: Float>(t534: F, t7350: F, t7349: F, t7353: F, t4617: F, t507: F, t1622: F, t1986: F, t7720: F, t321: F, t8924: F, t262: F) -> (F, F, F, F, F) {
    let t40717 = t7350 * t534;
    let t40719 = t7349 * t40717 * t7353;
    let t40724 = t507 * t4617;
    let t40731 = t1986 * t1622;
    let t40732 = t7720 * t40731;
    let t40734 = t8924 * t321;
    let t40735 = t262 * t40734;
    (t40719, t40724, t40732, t40734, t40735)
}
