//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 689/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk689<F: Float>(t2042: F, t592: F, t1867: F, t6405: F, t6407: F, t601: F, t1864: F, t586: F, t6347: F, t1847: F, t1859: F, t588: F) -> (F, F, F, F, F, F, F) {
    let t6811 = F::new(60.0) * t2042 * t592;
    let t6814 = t6405 * t6407 * t1867;
    let t6816 = F::cast_from(0.1038945353962551798e3_f64) * t601 * t6814;
    let t6820 = t1864 * t586;
    let t6821 = t6820 * t6347;
    let t6823 = F::cast_from(0.51947267698127589897e2_f64) * t601 * t6821;
    let t6825 = t1847 * t1859 * t588;
    (t6811, t6814, t6816, t6820, t6821, t6823, t6825)
}
