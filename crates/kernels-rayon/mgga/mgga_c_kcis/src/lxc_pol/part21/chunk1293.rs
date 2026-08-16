//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1293/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1293(t1092: f64, t27768: f64, t92917: f64, t13132: f64, t26685: f64, t26692: f64, t27954: f64, t27958: f64, t4939: f64, t7703: f64, t92740: f64, t93463: f64, t93468: f64, t93471: f64, t93562: f64, t95636: f64, t95783: f64, t95785: f64, t95798: f64, t95802: f64) -> (f64, f64) {
    let t95805 = t1092 * t92917 * t27768;
    let t95811 = t95783 - 0.46336805555555555556e-3_f64 * t7703 * t95785 - 0.16489724537037037037e-3_f64 * t93562 * t27958 + 0.18534722222222222223e-2_f64 * t7703 * t4939 * t93463 * t13132 - 0.12356481481481481482e-2_f64 * t26692 * t27954 + t95798 + 0.22109259259259259258e-2_f64 * t92740 + 0.99491666666666666664e-2_f64 * t95802 - 0.33163888888888888888e-2_f64 * t95805 - 0.6183646701388888889e-4_f64 * t93468 + 0.10297067901234567901e-3_f64 * t93471 - 0.18550940104166666667e-3_f64 * t26685 * t95636;
    (t95805, t95811)
}
