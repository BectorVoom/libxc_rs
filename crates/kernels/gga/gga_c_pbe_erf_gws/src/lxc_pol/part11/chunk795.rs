//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 795/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk795<F: Float>(t12854: F, t12868: F, t650: F, t186: F, t211: F, t1019: F, t3488: F, t7946: F, t11065: F, t7957: F, t11229: F, t11231: F, t12812: F, t12816: F, t12820: F, t12824: F, t12825: F, t12827: F, t12828: F, t12829: F, t12832: F, t12834: F, t231: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12869 = t12854 + t12868;
    let t12870 = t650 * t12869;
    let t12871 = t186 * t12870;
    let t12873 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t211 * t12871;
    let t12875 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t3488 * t1019;
    let t12876 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t7946;
    let t12877 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t11065;
    let t12878 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t7957;
    let t12879 = -t12812 + t12816 - t12820 + t12824 + t12825 + F::cast_from(4.0_f64) * t11229 + t12827 - t12828 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t12829 * t231 - t12832 + F::cast_from(4.0_f64) * t11231 + t12834 - t12873 - t12875 - t12876 + t12877 - t12878;
    (t12869, t12870, t12871, t12873, t12875, t12876, t12877, t12878, t12879)
}
