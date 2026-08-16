//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1964/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1964(t13042: f64, t17064: f64, t2054: f64, t259: f64, t26713: f64, t4142: f64, t4273: f64, t59503: f64, t7087: f64, t7823: f64, t7830: f64, t86870: f64, t92375: f64, t92382: f64, t92390: f64, t92393: f64, t98117: f64, t98122: f64, t98125: f64, t98135: f64, t98148: f64, t98153: f64, t98158: f64, t98164: f64, t98172: f64, t98181: f64) -> f64 {
    let t101335 = 0.15352717957250113407e0_f64 * t98117 - 0.9869604401089358619e-1_f64 * t98122 + 0.6579736267392905746e-1_f64 * t98125 + t92375 + 2.0_f64 * t4142 * t7823 * t259 + 4.0_f64 * t26713 * t4273 + t92382 - 0.20835831513410868196e0_f64 * t86870 - 0.16449340668482264365e-1_f64 * t98135 + t92390 + 0.3289868133696452873e-1_f64 * t98148 - 0.39478417604357434476e0_f64 * t98153 - 0.6579736267392905746e-1_f64 * t98158 + t92393 - t59503 * t2054 - 0.6579736267392905746e-1_f64 * t98164 - 6.0_f64 * t7087 * t17064 + 4.0_f64 * t13042 * t7830 + 0.9869604401089358619e-1_f64 * t98172 + 0.3289868133696452873e-1_f64 * t98181;
    t101335
}
