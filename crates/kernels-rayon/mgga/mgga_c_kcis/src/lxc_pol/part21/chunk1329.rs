//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1329/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1329(t1014: f64, t27879: f64, t303: f64, t4923: f64, t7731: f64, t1748: f64, t26832: f64, t27856: f64, t7687: f64, t27974: f64, t7696: f64, t10995: f64, t2175: f64, t26685: f64, t27958: f64, t4981: f64, t93211: f64, t93216: f64, t93366: f64, t95976: f64, t95980: f64, t95985: f64) -> (f64, f64, f64, f64) {
    let t96427 = t1014 * t27879;
    let t96428 = 0.33163888888888888888e-2_f64 * t96427;
    let t96430 = t303 * t4923 * t7731;
    let t96433 = t303 * t1748 * t26832;
    let t96449 = 0.46336805555555555556e-3_f64 * t7687 * t27856;
    let t96451 = 0.12356481481481481482e-2_f64 * t7696 * t27974;
    let t96452 = t96428 - 0.13265555555555555555e-1_f64 * t96430 + 0.24320185185185185185e-1_f64 * t96433 + 0.61836467013888888888e-4_f64 * t93366 * t27958 + 0.61836467013888888888e-4_f64 * t26685 * t95976 + 0.30918233506944444444e-4_f64 * t26685 * t95980 + 0.41224311342592592592e-4_f64 * t26685 * t95985 - 0.67960648148148148147e-2_f64 * t4981 * t10995 * t2175 + 0.16581944444444444444e-2_f64 * t93211 - 0.88437037037037037034e-2_f64 * t93216 + t96449 - t96451;
    (t96427, t96430, t96433, t96452)
}
