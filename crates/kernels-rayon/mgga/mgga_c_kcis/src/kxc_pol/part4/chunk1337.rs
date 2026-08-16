//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1337/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1337(t5857: f64, t738: f64, t5860: f64, t104: f64, t111: f64, t120: f64, t12066: f64, t17197: f64, t17199: f64, t17201: f64, t17203: f64, t17205: f64, t17207: f64, t17210: f64, t17213: f64, t17216: f64, t17219: f64, t17222: f64, t17225: f64, t17228: f64, t17231: f64, t17234: f64, t4059: f64, t4858: f64) -> f64 {
    let t17237 = t738 * t5857;
    let t17240 = 0.17611111111111111111e-2_f64 * t738 * t5860;
    let t17242 = 0.35222222222222222221e-2_f64 * t17197 + 0.39210208333333333333e-4_f64 * t17199 - 0.10929333333333333333e-1_f64 * t17201 + 0.77300125e-4_f64 * t17203 - 0.39814e-1_f64 * t17205 + 0.10038333333333333333e-1_f64 * t17207 - 0.23911438650126355246e-1_f64 * t4059 + 0.4755e-2_f64 * t111 * t17210 - 0.21078e-1_f64 * t104 * t17213 + 0.30247875e-4_f64 * t120 * t17216 + 0.7026e-2_f64 * t104 * t17219 - 0.7026e-2_f64 * t104 * t17222 - 0.28104e-1_f64 * t4858 * t17225 + 0.1171e-2_f64 * t104 * t17228 + 0.78066666666666666667e-3_f64 * t104 * t17231 - 0.4684e-2_f64 * t4858 * t17234 + 0.52833333333333333333e-2_f64 * t17237 + t17240 - 0.31077233446777841256e-3_f64 * t12066;
    t17242
}
