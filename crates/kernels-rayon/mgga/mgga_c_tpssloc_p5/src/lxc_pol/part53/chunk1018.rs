//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1018/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1018(t123570: f64, t123583: f64, t10110: f64, t114900: f64, t121629: f64, t121637: f64, t121648: f64, t121660: f64, t13042: f64, t218: f64, t24297: f64, t24305: f64, t259: f64, t2597: f64, t26690: f64, t26703: f64, t26713: f64, t31999: f64, t32018: f64, t33951: f64, t4147: f64, t4268: f64, t4300: f64, t7087: f64, t7092: f64, t7842: f64, t855: f64, t8733: f64, t8734: f64) -> (f64, f64) {
    let t123584 = t123570 + t123583;
    let t123612 = -0.76763589786250567037e-1_f64 * t121629 + t218 * t123584 * t259 + 4.0_f64 * t26713 * t7092 - 6.0_f64 * t855 * t10110 * t8733 * t4300 - 2.0_f64 * t24305 * t7842 - 6.0_f64 * t4268 * t32018 + 0.6579736267392905746e-1_f64 * t121637 + 4.0_f64 * t7087 * t26690 - 6.0_f64 * t2597 * t33951 + 0.15352717957250113407e0_f64 * t114900 + 2.0_f64 * t13042 * t8734 + 4.0_f64 * t7087 * t26703 + 0.6579736267392905746e-1_f64 * t121648 - 2.0_f64 * t24297 * t7842 - t4147 * t31999 + 0.76763589786250567037e-1_f64 * t121660;
    (t123584, t123612)
}
