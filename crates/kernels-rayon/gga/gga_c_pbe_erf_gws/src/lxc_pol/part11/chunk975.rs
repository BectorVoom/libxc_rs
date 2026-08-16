//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 975/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk975(t1673: f64, t3399: f64, t11190: f64, t2007: f64, t1697: f64, t3562: f64, t17197: f64, t3522: f64, t639: f64, t1672: f64, t211: f64, t3391: f64) -> (f64, f64, f64, f64, f64) {
    let t32093 = t3399 * t1673;
    let t32097 = t11190 * t2007;
    let t32114 = t3562 * t1697;
    let t32202 = t639 * t17197 * t3522;
    let t32215 = t211 * t1672 * t3391;
    (t32093, t32097, t32114, t32202, t32215)
}
