//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1158/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1158(t1017: f64, t10843: f64, t12480: f64, t12635: f64, t12794: f64, t12810: f64, t1809: f64, t1820: f64, t1821: f64, t1827: f64, t2615: f64, t30740: f64, t32629: f64, t3342: f64, t3415: f64, t42187: f64, t42189: f64, t42191: f64, t42204: f64, t47983: f64, t587: f64, t639: f64, t7130: f64) -> f64 {
    let t48423 = 32.0_f64 / 15.0_f64 * t42187 + 32.0_f64 / 27.0_f64 * t42189 + 64.0_f64 / 45.0_f64 * t42191 - 32.0_f64 / 15.0_f64 * t1820 * t1821 * t30740 * t3342 - 64.0_f64 / 15.0_f64 * t7130 * t12810 + 16.0_f64 / 15.0_f64 * t587 * t1827 * t32629 * t3342 - 32.0_f64 / 15.0_f64 * t10843 * t3415 + 16.0_f64 / 5.0_f64 * t639 * t1809 * t47983 - 16.0_f64 / 15.0_f64 * t2615 * t12635 - 16.0_f64 / 45.0_f64 * t587 * t1827 * t12480 * t1017 - 32.0_f64 / 15.0_f64 * t2615 * t12794 + 64.0_f64 / 45.0_f64 * t42204;
    t48423
}
