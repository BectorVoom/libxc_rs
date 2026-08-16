//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1109/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1109<F: Float>(t21624: F, t3902: F, t91: F, t80691: F, t992: F, t2354: F, t446: F, t4934: F, t4973: F, t9770: F, t4965: F, t41879: F) -> (F, F, F, F, F, F, F) {
    let t88218 = t91 * t3902 * t21624;
    let t88219 = t80691 * t992;
    let t88221 = t446 * t2354 * t88219;
    let t88223 = t4973 * t4934;
    let t88225 = t446 * t9770 * t88223;
    let t88227 = t4965 * t4934;
    let t88229 = t446 * t41879 * t88227;
    (t88218, t88219, t88221, t88223, t88225, t88227, t88229)
}
