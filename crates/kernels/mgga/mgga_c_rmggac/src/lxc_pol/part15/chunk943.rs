//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 943/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk943<F: Float>(t4044: F, t46055: F, t5058: F, t8639: F, t8642: F, t40759: F, t8646: F, t10007: F, t1356: F, t30490: F, t36280: F, t39320: F, t41978: F, t41980: F, t45556: F, t4601: F, t46072: F, t47757: F, t47759: F, t47761: F, t47765: F, t47767: F, t47772: F, t4985: F, t530: F, t7703: F, t884: F, t8866: F) -> (F,) {
    let t47774 = t4044 * t46055;
    let t47785 = t5058 * t8639 * t8642;
    let t47787 = t40759 * t8646;
    let t47791 = -0.4726e1 * t530 * t39320 + 0.11974241701863808564e0 * t4985 * t8866 - 0.85129199786595678796e-5 * t47757 + t41978 - t41980 + 0.18183107769496894486e-1 * t47759 + t47761 + 0.15961724959986689774e-4 * t47765 + 0.1064114997332445985e-4 * t47767 + 0.1064114997332445985e-4 * t47772 + 0.47896966807455234256e0 * t47774 + 0.79828278012425390428e-1 * t1356 * t46072 + 0.35922725105591425692e0 * t884 * t7703 * t30490 + 0.47896966807455234256e0 * t1356 * t36280 * t45556 + 0.40911992481368012592e-1 * t47785 - 0.81823984962736025184e-1 * t47787 + 0.35922725105591425692e0 * t4601 * t10007;
    (t47791,)
}
