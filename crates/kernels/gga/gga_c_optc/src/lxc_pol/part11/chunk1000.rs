//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1000/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1000<F: Float>(t2640: F, t31579: F, t4947: F, t2678: F, t2679: F, t40326: F, t4975: F, t7878: F, t893: F, t4961: F, t896: F, t4929: F, t530: F, t862: F, t2742: F, t2773: F, t5011: F) -> (F, F, F, F, F, F, F) {
    let t41526 = t2640 * t31579 * t4947;
    let t41585 = t2678 * t40326 * t2679;
    let t41756 = t7878 * t4975;
    let t41757 = t893 * t41756;
    let t41818 = t896 * t4961;
    let t41832 = t862 * t530 * t4929;
    let t41860 = t2773 * t2742 * t5011;
    (t41526, t41585, t41756, t41757, t41818, t41832, t41860)
}
