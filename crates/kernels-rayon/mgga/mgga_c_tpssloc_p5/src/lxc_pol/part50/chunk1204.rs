//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1204/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1204(t23384: f64, t32931: f64, t61: f64, t820: f64, t30838: f64, t354: f64, t373: f64, t10401: f64, t113361: f64, t113413: f64, t113443: f64, t23489: f64, t25589: f64, t25678: f64, t3033: f64, t30820: f64, t30827: f64, t32948: f64, t32951: f64, t4575: f64, t4579: f64, t4584: f64, t4589: f64, t4595: f64, t4599: f64, t6723: f64, t6742: f64, t8384: f64) -> (f64, f64) {
    let t119238 = t23384 * t32931;
    let t119243 = t820 * t61;
    let t119248 = t354 * t30838 * t373;
    let t119274 = -t3033 * t30827 * t10401 * t119243 * t4599 / 1536.0_f64 - t119248 * t119243 * t4584 / 1152.0_f64 + 5.0_f64 / 6912.0_f64 * t119248 * t119243 * t4589 + t113413 * t4575 / 2304.0_f64 + t113413 * t4579 / 2304.0_f64 + t3033 * t113443 * t10401 * t119243 * t4595 / 768.0_f64 + 0.32298204875312312685e-2_f64 * t6723 * t32948 + 0.40372756094140390856e-3_f64 * t25589 * t8384 + 0.40372756094140390856e-3_f64 * t23489 * t32951 + 0.40372756094140390856e-3_f64 * t6742 * t30820 * t25678 + t113361 / 2304.0_f64;
    (t119238, t119274)
}
