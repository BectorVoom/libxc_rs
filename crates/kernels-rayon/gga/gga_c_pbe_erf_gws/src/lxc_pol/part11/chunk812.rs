//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 812/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk812(t12803: f64, t12807: f64, t12812: f64, t12816: f64, t12820: f64, t12824: f64, t12825: f64, t12827: f64, t12828: f64, t12832: f64, t12834: f64, t12873: f64, t12875: f64, t12876: f64, t12877: f64, t12878: f64, t13039: f64, t267: f64, t5562: f64) -> f64 {
    let t13042 = t12803 - t12807 - t12812 + t12816 - t12820 + t12824 + t12825 + t12827 - t12828 - t12832 + t12834 - t12873 - t12875 - t12876 + t12877 + t5562 - t13039 * t267 / 15.0_f64 - t12878;
    t13042
}
