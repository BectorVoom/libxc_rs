//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1096/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1096<F: Float>(t2742: F, t2773: F, t5011: F, t40526: F, t953: F, t2672: F, t41818: F, t4941: F, t7212: F, t8384: F, t7467: F, t7481: F) -> (F, F, F, F, F, F, F) {
    let t41860 = t2773 * t2742 * t5011;
    let t41994 = t953 * t40526;
    let t42092 = t41818 * t2672;
    let t42111 = t7212 * t4941;
    let t42129 = t8384 * t4941;
    let t42136 = t7467 * t4941;
    let t42145 = t7481 * t4941;
    (t41860, t41994, t42092, t42111, t42129, t42136, t42145)
}
