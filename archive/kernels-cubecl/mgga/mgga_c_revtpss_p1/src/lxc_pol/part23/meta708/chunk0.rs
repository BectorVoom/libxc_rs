//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2462/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2462<F: Float>(t556: F, t786: F, t9656: F, t9303: F, t9641: F, t4146: F, t46279: F, t46291: F, t1892: F, t9646: F, t9648: F, t1904: F, t47567: F) -> (F, F, F, F, F, F, F) {
    let t47603 = t786 * t556 * t9656;
    let t47618 = t9303 * t9641;
    let t47671 = t4146 * t4146;
    let t47672 = F::cast_from(1.0_f64) / t47671;
    let t47753 = F::cast_from(36.0_f64) * t46279;
    let t47760 = F::cast_from(192.0_f64) * t46291;
    let t47764 = t9646 * t1892 * t9648;
    let t47772 = t47567 * t1904;
    (t47603, t47618, t47672, t47753, t47760, t47764, t47772)
}
