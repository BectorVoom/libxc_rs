//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1207/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1207(t1036: f64, t32954: f64, t25628: f64, t8384: f64, t30828: f64, t4640: f64, t1611: f64, t30839: f64, t23472: f64, t6753: f64, t7582: f64, t1025: f64, t1046: f64, t113416: f64, t113418: f64, t113429: f64, t113432: f64, t1610: f64, t25683: f64, t30821: f64, t30832: f64, t378: f64, t4615: f64, t8387: f64) -> f64 {
    let t119331 = t32954 * t1036;
    let t119335 = t25628 * t8384;
    let t119337 = t4640 * t30828;
    let t119340 = t1611 * t30839;
    let t119346 = t23472 * t6753 * t7582;
    let t119349 = t4615 * t8387 * t378 / 1536.0_f64 - t1610 * t30832 * t378 / 288.0_f64 + t119331 / 2304.0_f64 + 0.40372756094140390856e-3_f64 * t25683 * t30821 + 0.40372756094140390856e-3_f64 * t119335 + t119337 * t1025 / 1536.0_f64 + t119340 * t1046 / 2304.0_f64 - 0.32298204875312312685e-2_f64 * t113416 + 0.40372756094140390856e-3_f64 * t113418 + 0.40372756094140390856e-3_f64 * t119346 - t113429 / 432.0_f64 - t113432;
    t119349
}
