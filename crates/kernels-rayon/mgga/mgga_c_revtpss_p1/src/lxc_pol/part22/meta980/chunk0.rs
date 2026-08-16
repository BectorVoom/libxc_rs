//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3302/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3302(t23160: f64, t836: f64, t10529: f64, t2782: f64, t14520: f64, t14606: f64, t39576: f64, t39581: f64, t39586: f64, t39595: f64, t51298: f64, t62577: f64, t62583: f64, t62587: f64, t62591: f64, t62595: f64, t62601: f64) -> f64 {
    let t62604 = t23160 * t836;
    let t62606 = t2782 * t10529 * t62604;
    let t62609 = t14606 * t14520;
    let t62611 = -0.39029762157531132074e-1_f64 * t62577 - 0.23131639038696784278e-2_f64 * t39576 - 0.60712963356159538784e-1_f64 * t39581 + 0.13009920719177044025e-1_f64 * t39586 + 0.21951497276451705328e-1_f64 * t62583 - 0.19514881078765566038e-1_f64 * t62587 + 0.65854491829355115984e-1_f64 * t62591 - 0.65854491829355115984e-1_f64 * t62595 - 0.11708928647259339622e0_f64 * t62601 + 0.13009920719177044025e-1_f64 * t39595 - 0.21951497276451705328e-1_f64 * t62606 - 0.46263278077393568556e-2_f64 * t51298 - 0.39029762157531132075e-1_f64 * t62609;
    t62611
}
