//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 463/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk463<F: Float>(t435: F, t820: F, t869: F, t897: F, t6: F, t875: F, t2598: F, t2626: F) -> (F, F, F, F, F) {
    let t2770 = t435 * t820;
    let t2773 = t869 * t897;
    let t2776 = t6 * t875;
    let t2777 = t2598 * t2776;
    let t2778 = t2626 * t2777;
    (t2770, t2773, t2776, t2777, t2778)
}
