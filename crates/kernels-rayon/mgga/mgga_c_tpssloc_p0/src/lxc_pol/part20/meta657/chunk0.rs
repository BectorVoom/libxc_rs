//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2428/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2428(t13998: f64, t2960: f64, t42875: f64, t4338: f64, t973: f64, t10422: f64, t14040: f64, t3070: f64, t10516: f64, t4640: f64, t10403: f64, t14121: f64) -> (f64, f64, f64, f64, f64) {
    let t49658 = t2960 * t13998;
    let t49661 = t973 * t42875 * t4338;
    let t49662 = t49661 / 324.0_f64;
    let t49666 = t3070 * t10422 * t14040;
    let t49678 = t4640 * t10516;
    let t49682 = t10403 * t10422 * t14121;
    (t49658, t49662, t49666, t49678, t49682)
}
