//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 993/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk993(t1: f64, t5560: f64, t6006: f64, t119: f64, t1917: f64, t5559: f64, t17147: f64, t17150: f64, t17152: f64, t17154: f64, t17158: f64, t17160: f64, t17164: f64, t17167: f64, t17171: f64) -> f64 {
    let t18220 = t6006 * t1 * t5560;
    let t18224 = 0.60617527037037037035e-2_f64 * t5559 * t119 * t1917;
    let t18225 = -t17147 + t17150 - t17152 - t17154 + t17158 - t17160 + 0.60617527037037037035e-2_f64 * t18220 + t18224 + t17164 + t17167 + t17171;
    t18225
}
