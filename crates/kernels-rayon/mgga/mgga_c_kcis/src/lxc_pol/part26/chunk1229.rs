//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1229/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1229(t26654: f64, t838: f64, t26633: f64, t26652: f64, t26420: f64, t12286: f64, t491: f64, t990: f64, t3733: f64, t27368: f64, t61287: f64, t16968: f64, t3717: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93826 = t838 * t26654;
    let t93848 = 3.0_f64 * t26633;
    let t93849 = 3.0_f64 * t26652;
    let t93852 = 12.0_f64 * t26420;
    let t94208 = t12286 * t491 * t990;
    let t94216 = t3733 * t491;
    let t94227 = t27368 * t61287;
    let t94228 = t16968 * t3717;
    (t93826, t93848, t93849, t93852, t94208, t94216, t94227, t94228)
}
