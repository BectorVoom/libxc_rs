//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1037/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1037(t2447: f64, t664: f64, t321: f64, t5148: f64, t333: f64, t5266: f64, t558: f64, t71916: f64, t2367: f64, t698: f64, t352: f64, t8940: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t77960 = t2447 * t664;
    let t77963 = 0.11974241701863808564e0_f64 * t5148 * t77960 * t321;
    let t77966 = 0.11974241701863808564e0_f64 * t5266 * t77960 * t333;
    let t77969 = 0.11974241701863808564e0_f64 * t5266 * t71916 * t558;
    let t77970 = t698 * t2367;
    let t77973 = 0.11974241701863808564e0_f64 * t8940 * t77970 * t352;
    (t77960, t77963, t77966, t77969, t77970, t77973)
}
