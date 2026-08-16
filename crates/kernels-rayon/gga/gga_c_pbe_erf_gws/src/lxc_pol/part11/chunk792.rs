//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 792/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk792(t11054: f64, t954: f64, t4927: f64, t639: f64, t7845: f64, t11020: f64, t11023: f64, t12323: f64, t225: f64, t11026: f64, t11038: f64, t12497: f64, t1714: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12821 = t11054 * t954;
    let t12822 = t4927 * t12821;
    let t12824 = 8.0_f64 / 15.0_f64 * t639 * t12822;
    let t12825 = 4.0_f64 / 45.0_f64 * t7845;
    let t12827 = 32.0_f64 / 45.0_f64 * t11020;
    let t12828 = 16.0_f64 / 45.0_f64 * t11023;
    let t12829 = t12323 * t225;
    let t12832 = 4.0_f64 / 15.0_f64 * t11026;
    let t12834 = 8.0_f64 / 45.0_f64 * t11038;
    let t12837 = t1714 * t12497;
    (t12821, t12822, t12824, t12825, t12827, t12828, t12829, t12832, t12834, t12837)
}
