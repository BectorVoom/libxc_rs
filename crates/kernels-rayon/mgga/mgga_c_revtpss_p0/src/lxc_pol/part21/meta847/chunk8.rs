//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3183/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3183(t12555: f64, t5180: f64, t1168: f64, t12465: f64, t12472: f64, t12547: f64, t12553: f64, t16988: f64, t3471: f64, t3497: f64, t3515: f64, t3521: f64, t435: f64, t5120: f64, t5184: f64, t56260: f64, t58468: f64, t58472: f64, t58475: f64, t58477: f64, t58479: f64, t58481: f64, t58591: f64, t58592: f64, t58598: f64, t58618: f64, t58639: f64) -> f64 {
    let t58647 = t5180 * t12555;
    let t58654 = -t58468 + 1.0_f64 * t5120 * t12465 - t58472 - t58475 - t58477 - t58479 - t58481 - t58591 + 0.6207121550312808036e4_f64 * t58592 * t12472 * t3471 * t1168 - t58598 - 0.310907e-1_f64 * (t58618 + t58639) * t435 - 0.19751673498613801407e-1_f64 * t56260 + 0.51947577317044391277e2_f64 * t3521 * t16988 * t3515 + 0.30762056574649219973e4_f64 * t12553 * t58647 * t3497 + 0.17315859105681463759e2_f64 * t3521 * t5184 * t12547;
    t58654
}
