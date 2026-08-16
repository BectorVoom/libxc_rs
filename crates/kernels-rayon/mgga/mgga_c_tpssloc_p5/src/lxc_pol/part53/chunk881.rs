//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 881/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk881(t31349: f64, t10110: f64, t865: f64, t8733: f64, t31406: f64, t31425: f64, t2713: f64, t31330: f64, t31335: f64, t31340: f64, t31368: f64, t31371: f64, t31421: f64, t7087: f64, t7092: f64, t7107: f64, t855: f64, t8741: f64) -> (f64, f64, f64, f64, f64) {
    let t32014 = 0.76763589786250567037e-1_f64 * t31349;
    let t32018 = t10110 * t8733 * t865;
    let t32023 = 0.16449340668482264365e-1_f64 * t31406;
    let t32027 = 0.76763589786250567037e-1_f64 * t31425;
    let t32028 = -t2713 * t8741 - 0.3289868133696452873e-1_f64 * t31330 + 0.6579736267392905746e-1_f64 * t31335 + 0.6579736267392905746e-1_f64 * t31340 - t32014 - 0.6579736267392905746e-1_f64 * t31368 - 0.3289868133696452873e-1_f64 * t31371 - 6.0_f64 * t855 * t32018 - 2.0_f64 * t7087 * t7107 + t32023 - 0.3289868133696452873e-1_f64 * t31421 + 4.0_f64 * t7087 * t7092 + t32027;
    (t32014, t32018, t32023, t32027, t32028)
}
