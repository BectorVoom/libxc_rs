//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1236/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1236(t11333: f64, t40713: f64, t11345: f64, t11523: f64, t3718: f64, t5086: f64, t10998: f64, t3275: f64, t10610: f64, t3465: f64, t40644: f64, t11336: f64, t39263: f64, t39264: f64) -> (f64, f64, f64, f64, f64) {
    let t41788 = 5.0_f64 / 8.0_f64 * t40713 * t11333;
    let t41790 = t11523 * t11345 / 2.0_f64;
    let t41791 = t5086 * t3718;
    let t41794 = 45.0_f64 / 64.0_f64 * t3275 * t41791 * t10998;
    let t41797 = 3.0_f64 * t10610 * t3465 * t40644;
    let t41800 = 3.0_f64 * t39263 * t11336 * t39264;
    (t41788, t41790, t41794, t41797, t41800)
}
