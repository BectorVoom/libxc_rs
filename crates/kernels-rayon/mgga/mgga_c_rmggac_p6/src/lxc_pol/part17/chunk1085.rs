//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1085/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1085(t10007: f64, t1356: f64, t30490: f64, t36280: f64, t39320: f64, t41978: f64, t41980: f64, t45556: f64, t4601: f64, t46072: f64, t47757: f64, t47759: f64, t47761: f64, t47765: f64, t47767: f64, t47772: f64, t47774: f64, t47785: f64, t47787: f64, t4985: f64, t530: f64, t7703: f64, t884: f64, t8866: f64) -> f64 {
    let t47791 = -0.4726e1_f64 * t530 * t39320 + 0.11974241701863808564e0_f64 * t4985 * t8866 - 0.85129199786595678796e-5_f64 * t47757 + t41978 - t41980 + 0.18183107769496894486e-1_f64 * t47759 + t47761 + 0.15961724959986689774e-4_f64 * t47765 + 0.1064114997332445985e-4_f64 * t47767 + 0.1064114997332445985e-4_f64 * t47772 + 0.47896966807455234256e0_f64 * t47774 + 0.79828278012425390428e-1_f64 * t1356 * t46072 + 0.35922725105591425692e0_f64 * t884 * t7703 * t30490 + 0.47896966807455234256e0_f64 * t1356 * t36280 * t45556 + 0.40911992481368012592e-1_f64 * t47785 - 0.81823984962736025184e-1_f64 * t47787 + 0.35922725105591425692e0_f64 * t4601 * t10007;
    t47791
}
