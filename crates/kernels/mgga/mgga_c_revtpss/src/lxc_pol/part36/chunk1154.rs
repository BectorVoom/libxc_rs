//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1154/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1154<F: Float>(t21169: F, t7607: F, t29089: F, t5357: F, t21251: F, t21254: F, t20842: F, t7613: F, t1234: F, t30815: F, t20816: F, t7618: F, t29020: F, t5265: F, t20783: F, t26880: F) -> (F, F, F, F, F, F, F, F, F) {
    let t112397 = t7607 * t21169;
    let t112433 = t29089 * t5357;
    let t112435 = t7607 * t21251;
    let t112437 = t7607 * t21254;
    let t112452 = t7613 * t20842;
    let t112456 = t1234 * t30815;
    let t112461 = t7618 * t20816;
    let t112465 = t29020 * t5265;
    let t112468 = t26880 * t20783;
    (t112397, t112433, t112435, t112437, t112452, t112456, t112461, t112465, t112468)
}
