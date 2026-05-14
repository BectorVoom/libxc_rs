//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1175/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1175<F: Float>(t1517: F, t21130: F, t21134: F, t542: F, t4061: F, t6284: F, t1445: F, t6281: F, t12065: F, t532: F, t104: F, t12058: F, t12061: F, t12064: F, t17174: F, t17175: F, t17197: F, t17199: F, t17201: F, t17203: F, t17205: F, t17207: F, t17237: F, t17240: F, t4858: F) -> (F,) {
    let t21762 = t1517 * t21130;
    let t21765 = t542 * t21134;
    let t21775 = t4061 * t6284;
    let t21777 = t1445 * t6281;
    let t21779 = t12065 * t6281;
    let t21781 = t532 * t6284;
    let t21784 = 0.78066666666666666667e-3 * t104 * t21762 - 0.4684e-2 * t4858 * t21765 + t17174 + 0.31368166666666666667e-4 * t17175 - t12058 + t12061 + t12064 + 0.70444444444444444443e-2 * t17197 + 0.78420416666666666667e-4 * t17199 - 0.21858666666666666667e-1 * t17201 + 0.4705225e-4 * t17203 - 0.18736e-1 * t17205 + 0.52833333333333333332e-2 * t17207 + 0.10359077815592613752e-3 * t21775 + 0.23911438650126355246e-1 * t21777 - 0.31077233446777841256e-3 * t21779 - 0.11955719325063177623e-1 * t21781 + 0.52833333333333333332e-2 * t17237 + t17240;
    (t21784,)
}
