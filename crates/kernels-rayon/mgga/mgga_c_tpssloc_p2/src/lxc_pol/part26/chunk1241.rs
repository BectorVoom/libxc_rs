//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1241/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1241(t154: f64, t2690: f64, t3748: f64, t22691: f64, t12434: f64, t1985: f64, t1998: f64, t214: f64, t1887: f64, t22797: f64, t22734: f64, t1352: f64, t26331: f64, t3734: f64, t562: f64, t6976: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81151 = t2690 * t154;
    let t81152 = t81151 * t3748;
    let t81153 = t81152 * t22691;
    let t81154 = 0.98696044010893586188e-1_f64 * t81153;
    let t81157 = t1985 * t214 * t1998 * t12434;
    let t81159 = t22797 * t1887;
    let t81160 = t81159 * t22734;
    let t81165 = t26331 * t6976 * t562 * t3734 * t1352;
    (t81151, t81152, t81154, t81157, t81159, t81160, t81165)
}
