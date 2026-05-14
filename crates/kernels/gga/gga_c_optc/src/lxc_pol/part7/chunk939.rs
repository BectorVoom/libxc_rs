//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 939/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk939<F: Float>(t141: F, t2021: F, t2036: F, t2067: F, t22158: F, t22161: F, t22164: F, t22169: F, t22173: F, t22176: F, t22179: F, t22188: F, t22193: F, t22198: F, t22203: F, t22206: F, t22208: F, t22212: F, t22215: F, t636: F, t6876: F, t9642: F) -> (F,) {
    let t22217 = t22158 + 0.43465807448943789272e-1 * t636 * t22161 + 0.30426065214260652492e1 * t22164 + 0.13039742234683136782e1 * t636 * t22169 - 0.32599355586707841954e0 * t636 * t22173 - 0.60852130428521304982e0 * t22176 - 0.32599355586707841954e0 * t636 * t22179 + 0.13039742234683136782e0 * t9642 * t141 * t2067 * t2036 + 0.26079484469366273564e0 * t6876 * t22188 - 0.97798066760123525865e-1 * t6876 * t22193 - 0.26079484469366273564e0 * t2021 * t22198 - 0.10866451862235947318e-1 * t636 * t22203 - 0.60852130428521304982e0 * t22206 + 0.15213032607130326246e0 * t22208 + 0.21732903724471894636e-1 * t2021 * t22212 + 0.12170426085704260996e1 * t22215;
    (t22217,)
}
