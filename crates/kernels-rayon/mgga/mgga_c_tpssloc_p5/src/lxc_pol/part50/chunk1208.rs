//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1208/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1208(t119274: f64, t119303: f64, t119324: f64, t119349: f64, t23384: f64, t32928: f64, t1003: f64, t1022: f64, t1058: f64, t1060: f64, t113528: f64, t119238: f64, t23327: f64, t23346: f64, t25470: f64, t30878: f64, t30895: f64, t3180: f64, t32931: f64, t32938: f64, t32944: f64, t32961: f64, t32962: f64, t353: f64, t383: f64, t4669: f64, t6687: f64, t986: f64) -> (f64, f64) {
    let t119351 = t119274 + t119303 + t119324 + t119349;
    let t119357 = t23384 * t32928;
    let t119366 = 0.18277045187202515961e-2_f64 * t113528 + t3180 * t32944 + 0.43864908449286038307e-1_f64 * t23346 * t32931 - 0.54831135561607547883e-2_f64 * t119238 + t1003 * t32962 + t353 * t383 * t119351 - 0.54831135561607547883e-2_f64 * t23327 * t25470 * t30878 + 0.18277045187202515961e-2_f64 * t119357 - 0.16449340668482264365e-1_f64 * t6687 * t986 * t32938 + t4669 * t30895 + t1058 * t32961 * t1022 * t1060;
    (t119351, t119366)
}
