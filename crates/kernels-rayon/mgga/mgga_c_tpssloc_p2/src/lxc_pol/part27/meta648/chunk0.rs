//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2238/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2238(t7611: f64, t82713: f64, t82716: f64, t3040: f64, t7593: f64, t25550: f64, t82822: f64, t23384: f64, t25476: f64, t1058: f64, t1060: f64, t13940: f64, t14488: f64, t14618: f64, t1945: f64, t1953: f64, t23701: f64, t25499: f64, t25516: f64, t25535: f64, t2776: f64, t3186: f64, t3200: f64, t3201: f64, t4615: f64, t4673: f64, t6687: f64, t6784: f64, t6797: f64, t6813: f64, t7610: f64, t82592: f64, t986: f64) -> (f64, f64) {
    let t89309 = 0.14621636149762012769e-1_f64 * t82713 * t7611;
    let t89310 = t82716 * t7611;
    let t89312 = t7593 * t3040;
    let t89327 = 0.18277045187202515961e-2_f64 * t82822 * t25550;
    let t89329 = 0.18277045187202515961e-2_f64 * t23384 * t25476;
    let t89330 = -0.82246703342411321825e-2_f64 * t6797 * t82592 * t7610 + t1058 * t1945 * t14488 * t1060 - 0.54831135561607547884e-2_f64 * t6687 * t6784 * t25516 * t2776 - t89309 - 0.18277045187202515961e-2_f64 * t89310 - t3200 * t89312 * t3201 - 0.16449340668482264365e-1_f64 * t6687 * t986 * t25535 + t13940 * t1953 + 2.0_f64 * t4615 * t6813 + 4.0_f64 * t3186 * t25499 * t4673 + 2.0_f64 * t14618 * t23701 + t89327 + t89329;
    (t89312, t89330)
}
