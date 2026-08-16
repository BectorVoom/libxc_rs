//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1404/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1404(t22633: f64, t28116: f64, t90566: f64, t22635: f64, t26331: f64, t26332: f64, t6347: f64, t107314: f64, t107356: f64, t107391: f64, t107442: f64, t1375: f64, t1378: f64, t1843: f64, t20044: f64, t2006: f64, t20594: f64, t20612: f64, t26224: f64, t26225: f64, t26477: f64, t28224: f64, t5321: f64, t568: f64, t6440: f64, t7750: f64, t96913: f64, t97558: f64, t97664: f64) -> f64 {
    let t107460 = t22633 * t90566 * t28116;
    let t107464 = t26331 * t22635 * t26332 * t6347;
    let t107466 = -18.0_f64 * t5321 * t28224 - 3.0_f64 * t97558 * t1843 - t1375 * t1378 * (t107314 + t107356 + t107391 + t107442) - 0.34543615403812755166e0_f64 * t97664 - 3.0_f64 * t20044 * t7750 - 18.0_f64 * t26224 * t26225 * t20612 + 6.0_f64 * t26477 * t6440 + t20594 * t2006 * t568 - 3.0_f64 * t96913 * t1843 + 0.9869604401089358619e-1_f64 * t107460 + 0.14804406601634037928e0_f64 * t107464;
    t107466
}
