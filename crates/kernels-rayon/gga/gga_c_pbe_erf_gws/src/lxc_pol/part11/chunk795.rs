//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 795/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk795(t12854: f64, t12868: f64, t650: f64, t186: f64, t211: f64, t1019: f64, t3488: f64, t7946: f64, t11065: f64, t7957: f64, t11229: f64, t11231: f64, t12812: f64, t12816: f64, t12820: f64, t12824: f64, t12825: f64, t12827: f64, t12828: f64, t12829: f64, t12832: f64, t12834: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12869 = t12854 + t12868;
    let t12870 = t650 * t12869;
    let t12871 = t186 * t12870;
    let t12873 = 2.0_f64 / 15.0_f64 * t211 * t12871;
    let t12875 = 2.0_f64 / 5.0_f64 * t3488 * t1019;
    let t12876 = 8.0_f64 / 45.0_f64 * t7946;
    let t12877 = 8.0_f64 / 15.0_f64 * t11065;
    let t12878 = 8.0_f64 / 45.0_f64 * t7957;
    let t12879 = -t12812 + t12816 - t12820 + t12824 + t12825 + 4.0_f64 * t11229 + t12827 - t12828 + 4.0_f64 / 3.0_f64 * t12829 * t231 - t12832 + 4.0_f64 * t11231 + t12834 - t12873 - t12875 - t12876 + t12877 - t12878;
    (t12869, t12870, t12871, t12873, t12875, t12876, t12877, t12878, t12879)
}
