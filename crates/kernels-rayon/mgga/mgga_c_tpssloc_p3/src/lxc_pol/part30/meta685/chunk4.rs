//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2165/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2165(t22765: f64, t6417: f64, t6390: f64, t80997: f64, t81000: f64, t1351: f64, t3788: f64, t6388: f64, t6936: f64, t19958: f64, t22833: f64, t80867: f64, t80886: f64, t91304: f64, t91311: f64, t91323: f64, t91328: f64, t91345: f64, t91346: f64, t91357: f64, t91359: f64, t91365: f64, t93721: f64, t93723: f64) -> f64 {
    let t97378 = t22765 * t6417;
    let t97380 = t80997 * t6390;
    let t97382 = t81000 * t6390;
    let t97387 = t6936 * t3788 * t6388 * t1351;
    let t97389 = t22833 * t19958;
    let t97392 = -t91304 + t93721 + t91311 - t93723 - 119.0_f64 / 1728.0_f64 * t80867 + 7.0_f64 / 2304.0_f64 * t97378 - 7.0_f64 / 1152.0_f64 * t97380 + t97382 / 768.0_f64 + 0.20186378047070195427e-3_f64 * t91323 + t91328 + 0.12111826828242117256e-2_f64 * t97387 + t97389 / 384.0_f64 - t91345 + 0.33643963411783659045e-4_f64 * t91346 - t80886 - t91357 + t91359 - t91365;
    t97392
}
