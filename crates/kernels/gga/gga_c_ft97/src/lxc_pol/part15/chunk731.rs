//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 731/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk731<F: Float>(t4733: F, t925: F, t9144: F, t1017: F, t4417: F, t3440: F, t9115: F, t1053: F, t9121: F, t3439: F, t4431: F, t2222: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20743 = t4733 * t925;
    let t20744 = t9144 * t20743;
    let t20748 = t4417 * t1017;
    let t20749 = t3440 * t20748;
    let t20750 = t9115 * t20749;
    let t20753 = t4417 * t1053;
    let t20754 = t9121 * t20753;
    let t20755 = t3439 * t20754;
    let t20758 = t4431 * t1017;
    let t20759 = t2222 * t20758;
    (t20743, t20744, t20748, t20749, t20750, t20753, t20754, t20755, t20758, t20759)
}
