//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 934/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk934(t1517: f64, t21130: f64, t21134: f64, t542: f64, t4061: f64, t6284: f64, t1445: f64, t6281: f64, t12065: f64, t532: f64, t104: f64, t12058: f64, t12061: f64, t12064: f64, t17174: f64, t17175: f64, t17197: f64, t17199: f64, t17201: f64, t17203: f64, t17205: f64, t17207: f64, t17237: f64, t17240: f64, t4858: f64) -> f64 {
    let t21762 = t1517 * t21130;
    let t21765 = t542 * t21134;
    let t21775 = t4061 * t6284;
    let t21777 = t1445 * t6281;
    let t21779 = t12065 * t6281;
    let t21781 = t532 * t6284;
    let t21784 = 0.78066666666666666667e-3_f64 * t104 * t21762 - 0.4684e-2_f64 * t4858 * t21765 + t17174 + 0.31368166666666666667e-4_f64 * t17175 - t12058 + t12061 + t12064 + 0.70444444444444444443e-2_f64 * t17197 + 0.78420416666666666667e-4_f64 * t17199 - 0.21858666666666666667e-1_f64 * t17201 + 0.4705225e-4_f64 * t17203 - 0.18736e-1_f64 * t17205 + 0.52833333333333333332e-2_f64 * t17207 + 0.10359077815592613752e-3_f64 * t21775 + 0.23911438650126355246e-1_f64 * t21777 - 0.31077233446777841256e-3_f64 * t21779 - 0.11955719325063177623e-1_f64 * t21781 + 0.52833333333333333332e-2_f64 * t17237 + t17240;
    t21784
}
