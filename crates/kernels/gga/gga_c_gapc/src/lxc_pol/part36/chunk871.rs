//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 871/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk871<F: Float>(t1089: F, t11892: F, t3368: F, t3772: F, t1084: F, t11473: F, t3322: F, t11808: F, t3330: F, t11302: F, t7259: F, t8142: F, t2660: F) -> (F, F, F, F, F, F, F, F) {
    let t11893 = t11892 * t1089;
    let t11895 = t3772 * t3368;
    let t11897 = t1084 * t11473;
    let t11898 = t11897 * t3322;
    let t11900 = t11808 * t3330;
    let t11902 = t7259 * t11302;
    let t11903 = t11902 * t8142;
    let t11905 = t2660 * t11302;
    (t11893, t11895, t11897, t11898, t11900, t11902, t11903, t11905)
}
