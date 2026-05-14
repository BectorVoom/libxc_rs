//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 849/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk849<F: Float>(t1430: F, t16082: F, t16060: F, t542: F, t16078: F, t16069: F, t1517: F, t16055: F, t16065: F, t5857: F, t738: F, t5860: F, t104: F, t111: F, t120: F, t12066: F, t17197: F, t17199: F, t17201: F, t17203: F, t17205: F, t17207: F, t17210: F, t17213: F, t17216: F, t4059: F, t4858: F) -> (F,) {
    let t17219 = t1430 * t16082;
    let t17222 = t542 * t16060;
    let t17225 = t1430 * t16078;
    let t17228 = t542 * t16069;
    let t17231 = t1517 * t16055;
    let t17234 = t542 * t16065;
    let t17237 = t738 * t5857;
    let t17240 = 0.17611111111111111111e-2 * t738 * t5860;
    let t17242 = 0.35222222222222222221e-2 * t17197 + 0.39210208333333333333e-4 * t17199 - 0.10929333333333333333e-1 * t17201 + 0.77300125e-4 * t17203 - 0.39814e-1 * t17205 + 0.10038333333333333333e-1 * t17207 - 0.23911438650126355246e-1 * t4059 + 0.4755e-2 * t111 * t17210 - 0.21078e-1 * t104 * t17213 + 0.30247875e-4 * t120 * t17216 + 0.7026e-2 * t104 * t17219 - 0.7026e-2 * t104 * t17222 - 0.28104e-1 * t4858 * t17225 + 0.1171e-2 * t104 * t17228 + 0.78066666666666666667e-3 * t104 * t17231 - 0.4684e-2 * t4858 * t17234 + 0.52833333333333333333e-2 * t17237 + t17240 - 0.31077233446777841256e-3 * t12066;
    (t17242,)
}
