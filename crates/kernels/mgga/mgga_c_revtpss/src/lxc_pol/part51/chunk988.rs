//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 988/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk988<F: Float>(t31755: F, t33674: F, t1544: F, t2747: F, t31756: F, t31767: F, t1579: F, t31772: F, t4364: F, t233: F, t25373: F, t1559: F, t7076: F) -> (F, F, F, F, F, F, F, F) {
    let t33675 = t31755 * t33674;
    let t33678 = t2747 * t31756 * t1544;
    let t33679 = t31767 * t33678;
    let t33682 = t4364 * t31772 * t1579;
    let t33683 = t31767 * t33682;
    let t33687 = t233 * t1579;
    let t33688 = t25373 * t33687;
    let t33691 = t7076 * t1559;
    (t33675, t33678, t33679, t33682, t33683, t33687, t33688, t33691)
}
