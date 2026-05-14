//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 938/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk938<F: Float>(t28182: F, t8610: F, t11609: F, t1616: F, t687: F, t1734: F, t27622: F, t2660: F, t15483: F, t519: F, t9252: F, t1084: F, t9865: F, t11913: F, t28427: F, t435: F, t9281: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33144 = 12.0 * t28182 * t8610;
    let t33147 = 4.0 * t1616 * t11609 * t687;
    let t33148 = t1734 * t27622;
    let t33149 = t2660 * t33148;
    let t33150 = t33149 * t15483;
    let t33152 = t519 * t9252;
    let t33154 = t1084 * t33152 * t9865;
    let t33156 = t11913 * t28427;
    let t33158 = t435 * t9281;
    (t33144, t33147, t33148, t33149, t33150, t33152, t33154, t33156, t33158)
}
