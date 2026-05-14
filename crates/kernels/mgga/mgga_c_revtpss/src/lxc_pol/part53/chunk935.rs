//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 935/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk935<F: Float>(t1843: F, t8454: F, t13674: F, t8599: F, t2014: F, t1559: F, t31756: F, t4364: F, t31755: F, t1544: F, t2747: F, t31767: F, t1579: F, t31772: F, t233: F, t25373: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t33666 = 2.0 * t8454 * t1843;
    let t33667 = t8599 * t13674;
    let t33669 = 2.0 * t2014 * t33667;
    let t33674 = t4364 * t31756 * t1559;
    let t33675 = t31755 * t33674;
    let t33678 = t2747 * t31756 * t1544;
    let t33679 = t31767 * t33678;
    let t33682 = t4364 * t31772 * t1579;
    let t33683 = t31767 * t33682;
    let t33687 = t233 * t1579;
    let t33688 = t25373 * t33687;
    (t33666, t33667, t33669, t33674, t33675, t33678, t33679, t33682, t33683, t33687, t33688)
}
