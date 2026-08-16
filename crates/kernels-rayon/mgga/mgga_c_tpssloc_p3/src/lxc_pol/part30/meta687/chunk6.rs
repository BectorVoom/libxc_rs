//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2183/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2183(t1307: f64, t1377: f64, t22633: f64, t22635: f64, t6460: f64, t1375: f64, t1385: f64, t16030: f64, t1843: f64, t22656: f64, t22670: f64, t26348: f64, t26477: f64, t28111: f64, t28186: f64, t28220: f64, t3758: f64, t3882: f64, t3887: f64, t5321: f64, t5326: f64, t6440: f64, t7729: f64, t90732: f64, t91491: f64) -> f64 {
    let t97705 = t22633 * t22635 * t1377 * t6460 * t1307;
    let t97717 = 4.0_f64 * t5321 * t26348 + 2.0_f64 * t1375 * t3887 * t28186 * t1385 + 4.0_f64 * t16030 * t7729 + 4.0_f64 * t26477 * t5326 + 2.0_f64 * t3758 * t28111 + 0.16449340668482264365e-1_f64 * t97705 - 2.0_f64 * t91491 * t1843 - 2.0_f64 * t90732 * t1843 + 4.0_f64 * t3882 * t28220 + 2.0_f64 * t22670 * t6440 + 2.0_f64 * t22656 * t6440;
    t97717
}
