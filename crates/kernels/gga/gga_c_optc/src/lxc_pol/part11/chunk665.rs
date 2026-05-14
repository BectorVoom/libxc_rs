//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 665/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk665<F: Float>(t601: F, t6814: F, t1864: F, t586: F, t6347: F, t1847: F, t1859: F, t588: F, t518: F, t622: F, t84: F, t596: F, t120: F, t2086: F, t105: F, t2156: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6816 = 0.1038945353962551798e3 * t601 * t6814;
    let t6820 = t1864 * t586;
    let t6821 = t6820 * t6347;
    let t6823 = 0.51947267698127589897e2 * t601 * t6821;
    let t6825 = t1847 * t1859 * t588;
    let t6827 = 0.35089340384731224426e1 * t601 * t6825;
    let t6838 = t518 * t622 * t84;
    let t6840 = 0.56969282336565386482e-3 * t596 * t6838;
    let t6855 = t120 * t2086;
    let t6875 = t105 * t2156;
    (t6816, t6820, t6821, t6823, t6825, t6827, t6838, t6840, t6855, t6875)
}
