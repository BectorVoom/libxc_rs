//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1102/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1102<F: Float>(t5711: F, t786: F, t1364: F, t1357: F, t5775: F, t689: F, t2470: F, t5721: F, t3915: F, t1445: F, t5599: F, t2435: F, t5600: F) -> (F, F, F, F, F) {
    let t14082 = t786 * t5711;
    let t14084 = F::cast_from(0.19514881078765566038e-1_f64) * t14082 * t1364;
    let t14085 = t1357 * t5775;
    let t14087 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t14085;
    let t14090 = t5721 * t2470;
    let t14091 = t3915 * t14090;
    let t14094 = t5599 * t1445;
    let t14096 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t14094;
    let t14097 = t2435 * t5600;
    (t14084, t14087, t14091, t14096, t14097)
}
