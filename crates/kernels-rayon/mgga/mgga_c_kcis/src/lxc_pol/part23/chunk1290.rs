//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1290/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1290(t11418: f64, t1616: f64, t27607: f64, t28778: f64, t54162: f64, t7978: f64, t8225: f64, t27594: f64, t6140: f64, t16694: f64, t18183: f64, t27583: f64, t27598: f64, t28714: f64, t28835: f64, t8226: f64, t94905: f64, t94966: f64, t95021: f64, t98193: f64, t98201: f64, t99004: f64) -> f64 {
    let t99120 = t1616 * t11418;
    let t99129 = 0.23168402777777777778e-3_f64 * t27607 * t28778;
    let t99131 = t7978 * t54162 * t8225;
    let t99133 = t27594 * t6140;
    let t99144 = -0.36039737654320987655e-3_f64 * t27583 * t18183 * t99120 * t16694 + 0.185671721767578125e-4_f64 * t94966 * t99004 - 0.41270617283950617282e-2_f64 * t98193 + t99129 - 0.7722800925925925926e-4_f64 * t99131 + 0.24756229569010416667e-4_f64 * t99133 * t27598 - 0.30945286961263020833e-5_f64 * t94905 + 0.69644166666666666664e-2_f64 * t98201 - 0.69505208333333333334e-3_f64 * t28714 * t27598 + 0.34752604166666666667e-3_f64 * t95021 * t8226 + 0.69505208333333333334e-3_f64 * t27607 * t28835;
    t99144
}
