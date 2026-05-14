//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1404/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1404<F: Float>(t113307: F, t7724: F, t9406: F, t2707: F, t28178: F, t111201: F, t111203: F, t111206: F, t111221: F, t1156: F, t2351: F, t2709: F, t294: F, t35045: F, t35056: F, t35063: F, t5585: F, t9408: F) -> (F,) {
    let t120905 = 2.0 * t113307;
    let t120906 = t7724 * t9406;
    let t120907 = t28178 * t2707;
    let t120918 = -t111201 + t120905 + t111203 + t120906 + t120907 + t9408 * t35056 / 16.0 + t111206 - t2709 * t5585 * t2351 / 8.0 - t294 * t1156 * t35045 / 16.0 + t9408 * t35063 / 8.0 - t111221;
    (t120918,)
}
