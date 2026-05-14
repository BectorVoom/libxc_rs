//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 958/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk958<F: Float>(t1419: F, t7063: F, t116: F, t28159: F, t1892: F, t30: F, t41154: F, t1568: F, t33: F, t29421: F, t1518: F, t1936: F, t670: F, t7724: F, t8151: F, t84: F, t8440: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t94801 = t7063 * t1419;
    let t97622 = t28159 * t116;
    let t98040 = t7063 * t1892;
    let t98785 = t41154 * t30;
    let t98848 = t7063 * t1568;
    let t100981 = t41154 * t33;
    let t104115 = t29421 * t116;
    let t105823 = t1518 * t1936;
    let t108120 = t7724 * t670;
    let t111734 = t8151 * t670;
    let t119457 = t8440 * t84;
    (t94801, t97622, t98040, t98785, t98848, t100981, t104115, t105823, t108120, t111734, t119457)
}
