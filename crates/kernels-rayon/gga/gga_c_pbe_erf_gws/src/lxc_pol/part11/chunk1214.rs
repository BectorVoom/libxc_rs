//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1214/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1214(t12381: f64, t823: f64, t45660: f64, t9016: f64, t44296: f64, t11787: f64, t38264: f64, t3116: f64, t337: f64, t3703: f64, t3791: f64, t6560: f64) -> (f64, f64, f64, f64, f64) {
    let t49239 = t823 * t12381;
    let t49245 = 3.0_f64 / 4.0_f64 * t9016 * t45660;
    let t49259 = 7.0_f64 / 24.0_f64 * t44296;
    let t49273 = t38264 * t11787 / 8.0_f64;
    let t49279 = 3.0_f64 / 8.0_f64 * t3116 * t6560 * t337 * t3791 * t3703;
    (t49239, t49245, t49259, t49273, t49279)
}
