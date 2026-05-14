//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1285/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1285<F: Float>(t29851: F, t29853: F, t29855: F, t29857: F, t29860: F, t29862: F, t29865: F, t29867: F, t29870: F, t29873: F, t29877: F, t29880: F, t25342: F, t25345: F, t25348: F, t25359: F, t25362: F, t25365: F, t29884: F, t29888: F, t29892: F, t29896: F, t29905: F, t29909: F) -> (F, F) {
    let t30164 = -0.1898925e1 * t29851 - 0.9494625e0 * t29853 - 0.76790625e-1 * t29855 + 0.3071625e0 * t29857 + 0.3071625e0 * t29860 + 0.15358125e0 * t29862 + 0.5696775e1 * t29865 - 0.3071625e0 * t29867 + 0.27385555555555555555e0 * t29870 - 0.65725333333333333333e0 * t29873 + 0.49294e0 * t29877 - 0.32862666666666666666e0 * t29880;
    let t30177 = -0.32862666666666666666e0 * t29884 + 0.24647e0 * t29888 + 0.49294e0 * t29892 + 0.24647e0 * t29896 - 0.32862666666666666666e0 * t25342 - 0.65725333333333333332e0 * t25345 - 0.32862666666666666666e0 * t25348 - 0.14605629629629629629e1 * t25359 + 0.10954222222222222222e1 * t25362 + 0.10954222222222222222e1 * t25365 + 0.27385555555555555555e0 * t29905 + 0.49294e0 * t29909;
    (t30164, t30177)
}
