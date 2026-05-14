//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 868/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk868<F: Float>(t3342: F, t4951: F, t418: F, t5264: F, t2560: F, t34: F, t1856: F, t3421: F, t606: F, t2554: F, t4949: F, t11: F, t10778: F, t1758: F, t2704: F, t571: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10783 = t4951 * t3342;
    let t10784 = t10783 * t418;
    let t10785 = t5264 * t10784;
    let t10788 = t2560 * t34;
    let t10789 = t1856 * t10788;
    let t10792 = t3421 * t418;
    let t10793 = t606 * t10792;
    let t10796 = t2554 * t34;
    let t10797 = t606 * t10796;
    let t10800 = t4949 * t10784;
    let t10801 = t11 * t10800;
    let t10803 = t1758 * t10778;
    let t10804 = t11 * t10803;
    let t10806 = t1758 * t10788;
    let t10807 = t2704 * t10806;
    let t10809 = t571 * t10792;
    (t10784, t10785, t10788, t10789, t10792, t10793, t10796, t10797, t10801, t10804, t10807, t10809)
}
