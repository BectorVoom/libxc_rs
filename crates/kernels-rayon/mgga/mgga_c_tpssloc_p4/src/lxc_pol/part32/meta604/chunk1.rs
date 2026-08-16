//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1997/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1997(t81146: f64, t22642: f64, t22690: f64, t22881: f64, t154: f64, t2690: f64, t3748: f64, t22691: f64, t1887: f64, t22797: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81147 = 0.13707783890401886971e-2_f64 * t81146;
    let t81149 = t22642 * t22690 * t22881;
    let t81151 = t2690 * t154;
    let t81152 = t81151 * t3748;
    let t81153 = t81152 * t22691;
    let t81154 = 0.98696044010893586188e-1_f64 * t81153;
    let t81159 = t22797 * t1887;
    (t81147, t81149, t81151, t81152, t81154, t81159)
}
