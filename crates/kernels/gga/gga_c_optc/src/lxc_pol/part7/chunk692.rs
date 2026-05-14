//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 692/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk692<F: Float>(t1906: F, t591: F, t40: F, t2045: F, t559: F, t1979: F, t1983: F, t518: F, t622: F, t84: F, t596: F, t6480: F, t6484: F, t6488: F, t6492: F, t6816: F, t6819: F, t6823: F, t6827: F, t6829: F) -> (F, F, F, F, F, F, F) {
    let t6830 = t1906 * t591;
    let t6831 = t40 * t6830;
    let t6832 = 3.0 * t6831;
    let t6833 = t2045 * t559;
    let t6834 = 36.0 * t6833;
    let t6835 = t1979 * t1983;
    let t6836 = 0.73246220147012639764e-3 * t6835;
    let t6838 = t518 * t622 * t84;
    let t6840 = 0.56969282336565386482e-3 * t596 * t6838;
    let t6841 = t6816 - t6819 - t6480 - t6484 + t6488 - t6823 + t6827 + t6829 + t6832 + t6834 + t6836 + t6492 - t6840;
    (t6830, t6832, t6834, t6836, t6838, t6840, t6841)
}
