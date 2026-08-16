//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 793/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk793(t38704: f64, t16156: f64, t8808: f64, t8504: f64, t7345: f64, t8349: f64, t7335: f64, t7508: f64, t8533: f64, t2134: f64, t27: f64, t3118: f64, t551: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38705 = 0.17877131955185092547e-3_f64 * t38704;
    let t38710 = t16156 * t8808;
    let t38712 = t16156 * t8504;
    let t38749 = t7345 * t8349;
    let t38757 = t7335 * t8349;
    let t38775 = t7508 * t8533;
    let t38776 = 0.18183107769496894486e-1_f64 * t38775;
    let t38784 = t2134 * t27 * t3118 * t551;
    (t38705, t38710, t38712, t38749, t38757, t38776, t38784)
}
