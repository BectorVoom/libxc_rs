//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2941/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2941(t11977: f64, t4820: f64, t1042: f64, t1063: f64, t11859: f64, t15834: f64, t16076: f64, t16208: f64, t1675: f64, t19634: f64, t3117: f64, t3188: f64, t42195: f64, t42227: f64, t42230: f64, t42232: f64, t4806: f64, t53450: f64, t53459: f64, t53464: f64, t53473: f64, t53474: f64) -> f64 {
    let t53479 = t11977 * t4820;
    let t53490 = 0.14291339372689912324e-2_f64 * t3188 * t15834 + 0.71456696863449561621e-3_f64 * t1063 * t1042 * t4806 * t53459 + 0.71456696863449561621e-3_f64 * t1063 * t1042 * t4806 * t53464 + 0.19055119163586549765e-2_f64 * t1063 * t1042 * t16208 * t53450 + 0.23289590088828005269e-2_f64 * t1063 * t1042 * t53473 * t53474 - 0.45732285992607719436e-2_f64 * t53479 + 0.42874018118069736972e-3_f64 * t42227 + 0.14291339372689912324e-3_f64 * t42230 + 0.42874018118069736972e-3_f64 * t42232 - 0.22866142996303859718e-2_f64 * t42195 * t1675 - 0.12862205435420921092e-2_f64 * t11859 * t3117 * t16076 * t19634;
    t53490
}
