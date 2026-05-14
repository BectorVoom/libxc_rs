//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 377/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk377<F: Float>(t1849: F, t587: F, t1767: F, t1770: F, t1773: F, t1777: F, t1779: F, t1782: F) -> (F, F) {
    let t1850 = t1849 * t587;
    let t1859 = -0.57538888888888888889e0 * t1767 + 0.11507777777777777778e1 * t1770 + 0.40256666666666666667e0 * t1773 + 0.366775e-1 * t1777 + 0.73355e-1 * t1779 + 0.137975e0 * t1782;
    (t1850, t1859)
}
