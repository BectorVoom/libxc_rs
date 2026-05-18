//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 881/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk881<F: Float>(t1048: F, t4598: F, t808: F, t8720: F, t568: F, t739: F, t531: F, t3049: F, t769: F, t314: F, t313: F, t1035: F, t2154: F) -> (F, F, F, F, F, F, F, F) {
    let t8822 = t4598 * t1048;
    let t8827 = t808 * t8720;
    let t8828 = t568 * t8827;
    let t8833 = t739 * t8720;
    let t8834 = t531 * t8833;
    let t8841 = t769 * t3049;
    let t8844 = t314 * t8720;
    let t8845 = t313 * t8844;
    let t8848 = t2154 * t1035;
    (t8822, t8828, t8833, t8834, t8841, t8844, t8845, t8848)
}
