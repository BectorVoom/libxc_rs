//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1187/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1187<F: Float>(t20563: F, t5116: F, t9061: F, t3709: F, t3713: F, t5075: F, t11450: F, t11451: F, t21157: F, t34673: F, t34676: F, t34679: F, t34682: F, t34686: F, t34689: F, t34692: F, t34695: F) -> F {
    let t34698 = t9061 * t5116 * t20563;
    let t34701 = t3709 * t5075 * t3713;
    let t34704 = t11450 * t11451 * t21157;
    let t34706 = F::cast_from(0.34752370105806885418e-3_f64) * t34673 - F::cast_from(0.4637672555408563478e-4_f64) * t34676 + F::cast_from(0.34752370105806885418e-3_f64) * t34679 - F::cast_from(0.17632930253855266704e-5_f64) * t34682 - F::cast_from(0.2318836277704281739e-4_f64) * t34686 - F::cast_from(0.10821235962619981449e-3_f64) * t34689 + F::cast_from(0.36647919126739670507e-5_f64) * t34692 + F::cast_from(0.4419852458519115466e-7_f64) * t34695 - F::cast_from(0.23713668668337477784e-9_f64) * t34698 + F::cast_from(0.33148893438893365995e-7_f64) * t34701 - F::cast_from(0.14749912734985351565e-7_f64) * t34704;
    t34706
}
