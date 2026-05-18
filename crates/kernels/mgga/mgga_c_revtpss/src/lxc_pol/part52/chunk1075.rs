//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1075/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1075<F: Float>(t31755: F, t33674: F, t1544: F, t2747: F, t31756: F, t31767: F, t1579: F, t31772: F, t4364: F, t1568: F, t8477: F, t8485: F) -> (F, F, F, F, F, F, F) {
    let t33675 = t31755 * t33674;
    let t33678 = t2747 * t31756 * t1544;
    let t33679 = t31767 * t33678;
    let t33682 = t4364 * t31772 * t1579;
    let t33683 = t31767 * t33682;
    let t33695 = t8477 * t1568;
    let t33711 = t33695 * t8485;
    (t33675, t33678, t33679, t33682, t33683, t33695, t33711)
}
