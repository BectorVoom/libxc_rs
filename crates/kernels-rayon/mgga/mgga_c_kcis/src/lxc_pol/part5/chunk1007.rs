//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1007/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1007(t10108: f64, t1646: f64, t1030: f64, t3073: f64, t1072: f64, t4833: f64, t331: f64, t4837: f64, t1035: f64, t167: f64, t4845: f64, t1027: f64, t4849: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13600 = t10108 * t1646;
    let t13658 = t1030 * t3073;
    let t13665 = 0.93706135855523581992e-2_f64 * t1072 * t4833;
    let t13667 = 0.93706135855523581992e-2_f64 * t331 * t4837;
    let t13677 = t1035 * t167;
    let t13682 = 0.93706135855523581992e-2_f64 * t331 * t4845;
    let t13684 = 0.28111840756657074598e-1_f64 * t1027 * t4849;
    (t13600, t13658, t13665, t13667, t13677, t13682, t13684)
}
