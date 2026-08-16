//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 913/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk913(t1292: f64, t13: f64, t18515: f64, t18648: f64, t1276: f64, t1285: f64, t1291: f64, t1274: f64, t404: f64, t260: f64, t262: f64, t16578: f64, t88: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18651 = t1292 * t1292;
    let t18655 = 0.24954977986735470917e5_f64 * t13 / t18648 * t18515 / t18651;
    let t18658 = 36.0_f64 * t1291 * t1276 * t1285;
    let t18664 = t1285 * t1285;
    let t18667 = 6.0_f64 * t1274 * t18664 * t404;
    let t18670 = 1.0_f64 / t260;
    let t18684 = 1.0_f64 / t262;
    let t18708 = t16578 * t88;
    (t18655, t18658, t18664, t18667, t18670, t18684, t18708)
}
