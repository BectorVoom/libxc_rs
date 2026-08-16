//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1244/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1244(t98254: f64, t2242: f64, t4134: f64, t1386: f64, t16968: f64, t39052: f64, t491: f64, t990: f64, t1928: f64, t3964: f64, t52613: f64, t7908: f64, t8154: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98255 = 0.3684876543209876543e-2_f64 * t98254;
    let t98266 = t2242 * t4134;
    let t98270 = t16968 * t1386;
    let t98290 = t39052 * t491 * t990;
    let t98294 = t3964 * t1928 * t990;
    let t98308 = t7908 * t52613 * t8154;
    (t98255, t98266, t98270, t98290, t98294, t98308)
}
