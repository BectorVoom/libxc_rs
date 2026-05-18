//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1016/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1016<F: Float>(t162: F, t22202: F, t2030: F, t6928: F, t6778: F, t2024: F, t645: F, t6867: F, t6787: F, t6799: F, t141: F, t2021: F, t2036: F, t2067: F, t22158: F, t22161: F, t22164: F, t22169: F, t22173: F, t22176: F, t22179: F, t22188: F, t22193: F, t22198: F, t636: F, t6876: F, t9642: F) -> (F, F, F, F) {
    let t22203 = t162 * t22202;
    let t22206 = t2030 * t6928;
    let t22208 = t2030 * t6778;
    let t22211 = t6867 * t2024 * t645;
    let t22212 = t162 * t22211;
    let t22215 = t6799 * t6787;
    let t22217 = t22158 + F::new(0.43465807448943789272e-1) * t636 * t22161 + F::new(0.30426065214260652492e1) * t22164 + F::new(0.13039742234683136782e1) * t636 * t22169 - F::new(0.32599355586707841954e0) * t636 * t22173 - F::new(0.60852130428521304982e0) * t22176 - F::new(0.32599355586707841954e0) * t636 * t22179 + F::new(0.13039742234683136782e0) * t9642 * t141 * t2067 * t2036 + F::new(0.26079484469366273564e0) * t6876 * t22188 - F::new(0.97798066760123525865e-1) * t6876 * t22193 - F::new(0.26079484469366273564e0) * t2021 * t22198 - F::new(0.10866451862235947318e-1) * t636 * t22203 - F::new(0.60852130428521304982e0) * t22206 + F::new(0.15213032607130326246e0) * t22208 + F::new(0.21732903724471894636e-1) * t2021 * t22212 + F::new(0.12170426085704260996e1) * t22215;
    (t22203, t22211, t22212, t22217)
}
