//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 938/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk938<F: Float>(t108: F, t1557: F, t3188: F, t25615: F, t5618: F, t984: F, t28: F, t22563: F, t929: F, t7983: F, t22718: F, t6427: F, t22701: F, t938: F, t3099: F, t5522: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25616 = t108 * t1557;
    let t25617 = t25616 * t3188;
    let t25618 = t25615 * t25617;
    let t25621 = t5618 * t984;
    let t25622 = t28 * t25621;
    let t25625 = t22563 * t929;
    let t25626 = t7983 * t25625;
    let t25631 = t22718 * t6427;
    let t25637 = t22701 * t938;
    let t25640 = t5522 * t3099;
    (t25616, t25617, t25618, t25621, t25622, t25625, t25626, t25631, t25637, t25640)
}
