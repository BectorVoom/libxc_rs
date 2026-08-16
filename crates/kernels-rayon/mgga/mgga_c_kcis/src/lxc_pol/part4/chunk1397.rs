//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1397/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1397(t25: f64, t6184: f64, t1599: f64, t4429: f64, t6141: f64, t18119: f64, t5426: f64, t12617: f64, t5440: f64, t4440: f64, t12825: f64, t2099: f64) -> (f64, f64, f64, f64, f64) {
    let t18146 = t25 * t6184;
    let t18148 = t1599 * t18146 / 288.0_f64;
    let t18152 = t6141 * t4429 / 108.0_f64;
    let t18155 = t5426 * t18119;
    let t18156 = t12617 * t18155;
    let t18159 = t5440 * t18119;
    let t18160 = t4440 * t18159;
    let t18163 = t12825 * t2099;
    (t18148, t18152, t18156, t18160, t18163)
}
