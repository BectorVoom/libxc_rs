//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 937/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk937<F: Float>(t1104: F, t4914: F, t10524: F, t575: F, t2468: F, t3563: F, t1615: F, t3478: F, t11151: F, t883: F, t1117: F, t7062: F, t1734: F, t27622: F, t2660: F, t15483: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30523 = t1104 * t4914;
    let t30867 = t10524 * t575;
    let t31754 = t3563 * t2468;
    let t31767 = t3478 * t1615;
    let t31777 = t11151 * t883;
    let t31783 = t1117 * t7062;
    let t33148 = t1734 * t27622;
    let t33149 = t2660 * t33148;
    let t33150 = t33149 * t15483;
    (t30523, t30867, t31754, t31767, t31777, t31783, t33148, t33149, t33150)
}
