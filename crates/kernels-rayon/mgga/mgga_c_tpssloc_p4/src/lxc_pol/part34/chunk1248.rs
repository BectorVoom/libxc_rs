//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1248/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1248(t108029: f64, t108649: f64, t108780: f64, t108856: f64, t100996: f64, t107571: f64, t107634: f64, t1401: f64, t1458: f64, t16524: f64, t20162: f64, t20347: f64, t2039: f64, t2098: f64, t22445: f64, t22448: f64, t24465: f64, t27254: f64, t28893: f64, t28951: f64, t29422: f64, t29425: f64, t33185: f64, t3941: f64, t5371: f64, t5456: f64, t5493: f64, t55388: f64, t577: f64, t7230: f64, t75784: f64, t7801: f64, t7956: f64, t94170: f64) -> (f64, f64) {
    let t108858 = t108029 + t108649 + t108780 + t108856;
    let t108871 = 0.405e2_f64 * t100996 * t1458 + 81.0_f64 * t24465 * t22448 + 27.0_f64 * t2098 * t22445 + 0.405e2_f64 * t20162 * t7801 + 0.135e2_f64 * t1401 * t107634 + 81.0_f64 * t3941 * t28951 * t1458 + 81.0_f64 * t3941 * t7801 * t5493 + 81.0_f64 * t94170 * t5456 + 81.0_f64 * t28893 * t7801 + 0.405e2_f64 * t5371 * t28951 + 0.405e2_f64 * t27254 * t5493 + 81.0_f64 * t55388 * t7956 + 27.0_f64 * t3941 * t2039 * t20347 + 81.0_f64 * t33185 * t29425 + 0.45e1_f64 * t108858 * t577 + 81.0_f64 * t107571 * t2039 + 162.0_f64 * t16524 * t29422 + 81.0_f64 * t16524 * t29425 + 0.135e2_f64 * t7230 * t20347 + 0.135e2_f64 * t75784 * t2039;
    (t108858, t108871)
}
