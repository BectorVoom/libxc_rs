//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 740/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk740(t2145: f64, t3916: f64, t3759: f64, t6416: f64, t2319: f64, t3749: f64, t1109: f64, t857: f64, t858: f64, t856: f64, t1139: f64, t2169: f64) -> (f64, f64, f64, f64, f64) {
    let t12054 = t3916 * t2145;
    let t12057 = t6416 * t3759;
    let t12061 = t2319 * t3749;
    let t12064 = t857 * t858 * t1109;
    let t12065 = t856 * t12064;
    let t12068 = t2169 * t1139;
    (t12054, t12057, t12061, t12065, t12068)
}
