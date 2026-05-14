//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 561/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk561<F: Float>(t25610: F, t3188: F, t25609: F, t1308: F, t1642: F, t108: F, t1557: F, t5618: F, t984: F, t28: F, t22563: F, t929: F, t7983: F, t22718: F, t6427: F, t22701: F, t938: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25611 = t25610 * t3188;
    let t25612 = t25609 * t25611;
    let t25615 = t1642 * t1308;
    let t25616 = t108 * t1557;
    let t25617 = t25616 * t3188;
    let t25618 = t25615 * t25617;
    let t25621 = t5618 * t984;
    let t25622 = t28 * t25621;
    let t25625 = t22563 * t929;
    let t25626 = t7983 * t25625;
    let t25631 = t22718 * t6427;
    let t25637 = t22701 * t938;
    (t25611, t25612, t25615, t25617, t25618, t25622, t25626, t25631, t25637)
}
