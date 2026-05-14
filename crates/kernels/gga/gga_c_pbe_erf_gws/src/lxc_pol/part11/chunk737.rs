//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 737/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk737<F: Float>(t12870: F, t186: F, t211: F, t1019: F, t3488: F, t7946: F, t11065: F, t7957: F, t11229: F, t11231: F, t12812: F, t12816: F, t12820: F, t12824: F, t12825: F, t12827: F, t12828: F, t12829: F, t12832: F, t12834: F, t231: F) -> (F, F, F, F, F, F, F) {
    let t12871 = t186 * t12870;
    let t12873 = 2.0 / 15.0 * t211 * t12871;
    let t12875 = 2.0 / 5.0 * t3488 * t1019;
    let t12876 = 8.0 / 45.0 * t7946;
    let t12877 = 8.0 / 15.0 * t11065;
    let t12878 = 8.0 / 45.0 * t7957;
    let t12879 = -t12812 + t12816 - t12820 + t12824 + t12825 + 4.0 * t11229 + t12827 - t12828 + 4.0 / 3.0 * t12829 * t231 - t12832 + 4.0 * t11231 + t12834 - t12873 - t12875 - t12876 + t12877 - t12878;
    (t12871, t12873, t12875, t12876, t12877, t12878, t12879)
}
