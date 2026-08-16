//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1186/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1186(t20563: f64, t5116: f64, t9061: f64, t3709: f64, t3713: f64, t5075: f64, t11450: f64, t11451: f64, t21157: f64, t34673: f64, t34676: f64, t34679: f64, t34682: f64, t34686: f64, t34689: f64, t34692: f64, t34695: f64) -> f64 {
    let t34698 = t9061 * t5116 * t20563;
    let t34701 = t3709 * t5075 * t3713;
    let t34704 = t11450 * t11451 * t21157;
    let t34706 = 0.34752370105806885418e-3_f64 * t34673 - 0.4637672555408563478e-4_f64 * t34676 + 0.34752370105806885418e-3_f64 * t34679 - 0.17632930253855266704e-5_f64 * t34682 - 0.2318836277704281739e-4_f64 * t34686 - 0.10821235962619981449e-3_f64 * t34689 + 0.36647919126739670507e-5_f64 * t34692 + 0.4419852458519115466e-7_f64 * t34695 - 0.23713668668337477784e-9_f64 * t34698 + 0.33148893438893365995e-7_f64 * t34701 - 0.14749912734985351565e-7_f64 * t34704;
    t34706
}
