//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 963/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk963<F: Float>(t17817: F, t197: F, t4991: F, t1820: F, t1823: F, t1672: F, t1871: F, t561: F, t5280: F, t579: F, t1733: F, t184: F, t209: F, t221: F) -> (F, F, F, F, F) {
    let t17818 = F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t17817;
    let t17819 = t4991 * t197;
    let t17821 = t1820 * t17819 * t1823;
    let t17822 = F::cast_from(64.0_f64) / F::cast_from(135.0_f64) * t17821;
    let t17824 = t561 * t1672 * t1871;
    let t17825 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t17824;
    let t17826 = t579 * t5280;
    let t17827 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t17826;
    let t17828 = t1733 * t1733;
    let t17832 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t17828 * t209 * t184 * t221;
    (t17818, t17822, t17825, t17827, t17832)
}
