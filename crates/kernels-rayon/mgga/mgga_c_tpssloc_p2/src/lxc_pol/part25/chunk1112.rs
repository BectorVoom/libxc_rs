//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1112/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1112(t22642: f64, t22690: f64, t22881: f64, t154: f64, t2690: f64, t3748: f64, t22691: f64, t12434: f64, t1985: f64, t1998: f64, t214: f64, t1887: f64, t22797: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81149 = t22642 * t22690 * t22881;
    let t81151 = t2690 * t154;
    let t81152 = t81151 * t3748;
    let t81153 = t81152 * t22691;
    let t81157 = t1985 * t214 * t1998 * t12434;
    let t81159 = t22797 * t1887;
    (t81149, t81151, t81152, t81153, t81157, t81159)
}
