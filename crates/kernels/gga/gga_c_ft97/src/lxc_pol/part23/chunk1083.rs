//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1083/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1083<F: Float>(t17817: F, t65693: F, t17980: F, t7853: F, t30635: F, t709: F, t226: F, t4986: F, t5025: F, t679: F, t689: F, t5001: F, t17840: F, t17976: F, t1613: F, t1689: F) -> (F, F, F, F, F, F, F, F) {
    let t79641 = t17817 * t65693;
    let t79818 = t7853 * t17980;
    let t79821 = t30635 * t709;
    let t79851 = t4986 * t226;
    let t79854 = t5025 * t679;
    let t79855 = t79854 * t689;
    let t79862 = t5001 * t679;
    let t79864 = t17840 * t79862 * t689;
    let t79911 = t7853 * t17976;
    let t79931 = t1689 * t1613;
    (t79641, t79818, t79821, t79851, t79855, t79864, t79911, t79931)
}
