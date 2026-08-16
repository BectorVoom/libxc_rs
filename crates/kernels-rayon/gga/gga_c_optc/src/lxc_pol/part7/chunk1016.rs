//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1016/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1016(t162: f64, t22202: f64, t2030: f64, t6928: f64, t6778: f64, t2024: f64, t645: f64, t6867: f64, t6787: f64, t6799: f64, t141: f64, t2021: f64, t2036: f64, t2067: f64, t22158: f64, t22161: f64, t22164: f64, t22169: f64, t22173: f64, t22176: f64, t22179: f64, t22188: f64, t22193: f64, t22198: f64, t636: f64, t6876: f64, t9642: f64) -> (f64, f64, f64, f64) {
    let t22203 = t162 * t22202;
    let t22206 = t2030 * t6928;
    let t22208 = t2030 * t6778;
    let t22211 = t6867 * t2024 * t645;
    let t22212 = t162 * t22211;
    let t22215 = t6799 * t6787;
    let t22217 = t22158 + 0.43465807448943789272e-1_f64 * t636 * t22161 + 0.30426065214260652492e1_f64 * t22164 + 0.13039742234683136782e1_f64 * t636 * t22169 - 0.32599355586707841954e0_f64 * t636 * t22173 - 0.60852130428521304982e0_f64 * t22176 - 0.32599355586707841954e0_f64 * t636 * t22179 + 0.13039742234683136782e0_f64 * t9642 * t141 * t2067 * t2036 + 0.26079484469366273564e0_f64 * t6876 * t22188 - 0.97798066760123525865e-1_f64 * t6876 * t22193 - 0.26079484469366273564e0_f64 * t2021 * t22198 - 0.10866451862235947318e-1_f64 * t636 * t22203 - 0.60852130428521304982e0_f64 * t22206 + 0.15213032607130326246e0_f64 * t22208 + 0.21732903724471894636e-1_f64 * t2021 * t22212 + 0.12170426085704260996e1_f64 * t22215;
    (t22203, t22211, t22212, t22217)
}
