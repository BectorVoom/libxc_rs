//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 795/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk795<F: Float>(t2722: F, t7983: F, t2434: F, t2670: F, t7494: F, t7452: F, t2641: F, t2644: F, t2813: F, t1: F, t2672: F, t770: F, t7845: F, t953: F, t7848: F, t2367: F, t2629: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7984 = t2722 * t7983;
    let t7987 = t2434 * t2670;
    let t7988 = t7987 * t7494;
    let t7992 = t7987 * t7452;
    let t7995 = t2641 * t2670;
    let t7996 = t7995 * t2644;
    let t7999 = t2813 * t7983;
    let t8002 = t2672 * t1;
    let t8003 = t8002 * t770;
    let t8004 = t7995 * t8003;
    let t8007 = t953 * t7845;
    let t8009 = t953 * t7848;
    let t8019 = t2367 * t2629;
    (t7984, t7987, t7988, t7992, t7995, t7996, t7999, t8002, t8003, t8004, t8007, t8009, t8019)
}
