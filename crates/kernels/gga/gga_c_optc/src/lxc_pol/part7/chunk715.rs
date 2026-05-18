//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 715/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk715<F: Float>(t1867: F, t6405: F, t6407: F, t601: F, t1: F, t1906: F, t598: F, t1864: F, t586: F, t6347: F, t1847: F, t1859: F, t588: F) -> (F, F, F, F, F, F, F, F) {
    let t6814 = t6405 * t6407 * t1867;
    let t6816 = F::new(0.1038945353962551798e3) * t601 * t6814;
    let t6817 = t1906 * t1;
    let t6818 = t6817 * t598;
    let t6819 = F::new(0.54934665110259479823e-3) * t6818;
    let t6820 = t1864 * t586;
    let t6821 = t6820 * t6347;
    let t6823 = F::new(0.51947267698127589897e2) * t601 * t6821;
    let t6825 = t1847 * t1859 * t588;
    (t6814, t6816, t6817, t6819, t6820, t6821, t6823, t6825)
}
