//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1627/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1627<F: Float>(t17691: F, t4588: F, t4582: F, t14187: F, t17686: F, t5878: F, t884: F, t3071: F, t1616: F, t4347: F, t376: F, t5866: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17692 = t4588 * t17691;
    let t17693 = t4582 * t17692;
    let t17696 = t14187 * t17686;
    let t17697 = t4582 * t17696;
    let t17700 = t5878 * t884;
    let t17701 = t3071 * t17700;
    let t17704 = t1616 * t4347;
    let t17705 = t3071 * t17704;
    let t17712 = t376 * t5866;
    (t17692, t17693, t17696, t17697, t17700, t17701, t17704, t17705, t17712)
}
