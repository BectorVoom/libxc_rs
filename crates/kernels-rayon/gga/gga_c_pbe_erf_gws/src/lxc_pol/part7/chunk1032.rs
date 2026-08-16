//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1032/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1032(t1321: f64, t18638: f64, t18639: f64, t470: f64, t1396: f64, t4793: f64, t1289: f64, t1292: f64, t13: f64, t18515: f64, t1276: f64, t1285: f64, t1291: f64) -> (f64, f64, f64, f64, f64) {
    let t18641 = t1321 * t1321;
    let t18642 = 1.0_f64 / t18641;
    let t18645 = 0.91080982599109921211e5_f64 * t470 * t18638 * t18639 * t18642;
    let t18646 = t4793 * t1396;
    let t18647 = 0.35089340384731224426e1_f64 * t18646;
    let t18648 = t1289 * t1289;
    let t18651 = t1292 * t1292;
    let t18655 = 0.24954977986735470917e5_f64 * t13 / t18648 * t18515 / t18651;
    let t18658 = 36.0_f64 * t1291 * t1276 * t1285;
    (t18642, t18645, t18647, t18655, t18658)
}
