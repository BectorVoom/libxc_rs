//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1189/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1189<F: Float>(t2747: F, t31756: F, t31767: F, t4343: F, t10779: F, t119837: F, t1544: F, t119968: F, t119836: F, t119875: F, t33678: F, t119781: F, t1579: F, t247: F, t837: F, t867: F) -> (F, F, F, F, F) {
    let t126319 = t31767 * t2747 * t31756 * t4343;
    let t126322 = t10779 * t119837 * t1544;
    let t126323 = t119968 * t126322;
    let t126325 = t119836 * t126322;
    let t126327 = t119875 * t33678;
    let t126340 = t119781 * t247 * t867 * t1579 * t837;
    (t126319, t126323, t126325, t126327, t126340)
}
