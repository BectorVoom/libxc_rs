//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1782/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1782<F: Float>(t47530: F, t9682: F, t2439: F, t3895: F, t4132: F, t1357: F, t689: F, t9659: F, t3899: F, t4131: F, t10175: F, t9671: F) -> (F, F, F, F, F, F) {
    let t47531 = t47530 * t9682;
    let t47534 = t2439 * t3895 * t4132;
    let t47537 = t689 * t1357 * t9659;
    let t47540 = t689 * t3899 * t4132;
    let t47546 = t4131 * t4131;
    let t47550 = t10175 * t9671;
    (t47531, t47534, t47537, t47540, t47546, t47550)
}
