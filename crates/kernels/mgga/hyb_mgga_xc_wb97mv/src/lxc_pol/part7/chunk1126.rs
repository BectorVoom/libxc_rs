//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1126/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1126<F: Float>(t1979: F, t554: F, t6432: F, t1984: F, t10: F, t6147: F, t1995: F, t2003: F, t1993: F, t1997: F, t1988: F, t136: F, t2164: F, t6528: F, t2049: F, t6715: F, t683: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21796 = t554 * t6432 * t1979;
    let t21799 = t554 * t6432 * t1984;
    let t21871 = t6147 * t10;
    let t21885 = t2003 * t1995;
    let t21887 = t1993 * t21885 * t1997;
    let t21890 = t554 * t6432 * t1988;
    let t21910 = t136 * t2003 * t2164;
    let t21924 = t6528 * t10;
    let t21949 = t683 * t6715 * t2049;
    (t21796, t21799, t21871, t21885, t21887, t21890, t21910, t21924, t21949)
}
