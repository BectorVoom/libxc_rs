//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3302/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3302<F: Float>(t23160: F, t836: F, t10529: F, t2782: F, t14520: F, t14606: F, t39576: F, t39581: F, t39586: F, t39595: F, t51298: F, t62577: F, t62583: F, t62587: F, t62591: F, t62595: F, t62601: F) -> F {
    let t62604 = t23160 * t836;
    let t62606 = t2782 * t10529 * t62604;
    let t62609 = t14606 * t14520;
    let t62611 = -F::cast_from(0.39029762157531132074e-1_f64) * t62577 - F::cast_from(0.23131639038696784278e-2_f64) * t39576 - F::cast_from(0.60712963356159538784e-1_f64) * t39581 + F::cast_from(0.13009920719177044025e-1_f64) * t39586 + F::cast_from(0.21951497276451705328e-1_f64) * t62583 - F::cast_from(0.19514881078765566038e-1_f64) * t62587 + F::cast_from(0.65854491829355115984e-1_f64) * t62591 - F::cast_from(0.65854491829355115984e-1_f64) * t62595 - F::cast_from(0.11708928647259339622e0_f64) * t62601 + F::cast_from(0.13009920719177044025e-1_f64) * t39595 - F::cast_from(0.21951497276451705328e-1_f64) * t62606 - F::cast_from(0.46263278077393568556e-2_f64) * t51298 - F::cast_from(0.39029762157531132075e-1_f64) * t62609;
    t62611
}
