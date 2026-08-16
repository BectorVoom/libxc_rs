//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1337/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1337<F: Float>(t5857: F, t738: F, t5860: F, t104: F, t111: F, t120: F, t12066: F, t17197: F, t17199: F, t17201: F, t17203: F, t17205: F, t17207: F, t17210: F, t17213: F, t17216: F, t17219: F, t17222: F, t17225: F, t17228: F, t17231: F, t17234: F, t4059: F, t4858: F) -> F {
    let t17237 = t738 * t5857;
    let t17240 = F::cast_from(0.17611111111111111111e-2_f64) * t738 * t5860;
    let t17242 = F::cast_from(0.35222222222222222221e-2_f64) * t17197 + F::cast_from(0.39210208333333333333e-4_f64) * t17199 - F::cast_from(0.10929333333333333333e-1_f64) * t17201 + F::cast_from(0.77300125e-4_f64) * t17203 - F::cast_from(0.39814e-1_f64) * t17205 + F::cast_from(0.10038333333333333333e-1_f64) * t17207 - F::cast_from(0.23911438650126355246e-1_f64) * t4059 + F::cast_from(0.4755e-2_f64) * t111 * t17210 - F::cast_from(0.21078e-1_f64) * t104 * t17213 + F::cast_from(0.30247875e-4_f64) * t120 * t17216 + F::cast_from(0.7026e-2_f64) * t104 * t17219 - F::cast_from(0.7026e-2_f64) * t104 * t17222 - F::cast_from(0.28104e-1_f64) * t4858 * t17225 + F::cast_from(0.1171e-2_f64) * t104 * t17228 + F::cast_from(0.78066666666666666667e-3_f64) * t104 * t17231 - F::cast_from(0.4684e-2_f64) * t4858 * t17234 + F::cast_from(0.52833333333333333333e-2_f64) * t17237 + t17240 - F::cast_from(0.31077233446777841256e-3_f64) * t12066;
    t17242
}
