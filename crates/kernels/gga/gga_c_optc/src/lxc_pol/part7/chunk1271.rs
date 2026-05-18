//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1271/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1271<F: Float>(t2985: F, t2992: F, t2995: F, t1032: F, t8581: F, t8583: F, t26153: F, t3020: F, t8686: F, t1036: F, t8896: F, t1057: F) -> (F, F, F, F) {
    let t26201 = t2985 * t2992;
    let t26203 = F::new(12.0) * t26201 * t2995;
    let t26204 = t1032 * t8581;
    let t26206 = F::new(0.38596378373162651572e3) * t26204 * t8583;
    let t26209 = F::new(0.57894567559743977359e3) * t8686 * t26153 * t3020;
    let t26210 = t8896 * t1036;
    let t26212 = F::new(4.0) * t26210 * t1057;
    (t26203, t26206, t26209, t26212)
}
