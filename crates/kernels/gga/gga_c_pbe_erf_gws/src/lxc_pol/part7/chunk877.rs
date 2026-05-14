//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 877/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk877<F: Float>(t17824: F, t5280: F, t579: F, t1733: F, t184: F, t209: F, t221: F, t1748: F, t1781: F, t1735: F, t5343: F, t17807: F, t17809: F, t17813: F, t17815: F, t17818: F, t17822: F) -> (F, F, F, F, F, F) {
    let t17825 = 16.0 / 45.0 * t17824;
    let t17826 = t579 * t5280;
    let t17827 = 16.0 / 15.0 * t17826;
    let t17828 = t1733 * t1733;
    let t17832 = 4.0 / 5.0 * t17828 * t209 * t184 * t221;
    let t17836 = 8.0 / 5.0 * t1781 * t1748 * t184 * t221;
    let t17838 = 8.0 / 5.0 * t5343 * t1735;
    let t17839 = -t17807 + t17809 + t17813 - t17815 + t17818 - t17822 - t17825 - t17827 + t17832 + t17836 + t17838;
    (t17825, t17827, t17832, t17836, t17838, t17839)
}
