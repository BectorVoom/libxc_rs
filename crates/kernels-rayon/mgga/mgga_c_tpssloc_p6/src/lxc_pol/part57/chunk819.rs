//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 819/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk819(t29009: f64, t29054: f64, t858: f64, t2053: f64, t2718: f64, t5657: f64, t218: f64, t29040: f64, t1528: f64, t17090: f64, t2054: f64, t25036: f64, t25049: f64, t259: f64, t26713: f64, t28265: f64, t28269: f64, t28274: f64, t28278: f64, t28289: f64, t28296: f64, t28300: f64, t4147: f64, t4268: f64, t5637: f64, t7087: f64, t7830: f64, t855: f64) -> (f64, f64, f64, f64, f64) {
    let t29055 = t29009 + t29054;
    let t29056 = t858 * t29055;
    let t29060 = t2718 * t2053 * t5657;
    let t29071 = t218 * t29040;
    let t29075 = -0.16449340668482264365e-1_f64 * t25036 - 0.16449340668482264365e-1_f64 * t28265 + 0.6579736267392905746e-1_f64 * t28269 - 0.76763589786250567036e-1_f64 * t25049 + 0.16449340668482264365e-1_f64 * t28274 + 4.0_f64 * t4268 * t7830 - t855 * t29056 - 0.3289868133696452873e-1_f64 * t28278 + 2.0_f64 * t855 * t29060 - t17090 * t2054 - 0.6579736267392905746e-1_f64 * t28289 + 2.0_f64 * t7087 * t5637 - 2.0_f64 * t26713 * t1528 + 0.3289868133696452873e-1_f64 * t28296 + 0.9869604401089358619e-1_f64 * t28300 + t29071 * t259 + 4.0_f64 * t4147 * t7830;
    (t29055, t29056, t29060, t29071, t29075)
}
