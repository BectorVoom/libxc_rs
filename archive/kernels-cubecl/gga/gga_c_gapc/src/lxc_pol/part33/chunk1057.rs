//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1057/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1057<F: Float>(t28182: F, t8610: F, t11609: F, t1616: F, t687: F, t1734: F, t27622: F, t2660: F, t15483: F, t519: F, t9252: F, t1084: F, t9865: F) -> (F, F, F, F, F, F, F) {
    let t33144 = F::cast_from(12.0_f64) * t28182 * t8610;
    let t33147 = F::cast_from(4.0_f64) * t1616 * t11609 * t687;
    let t33148 = t1734 * t27622;
    let t33149 = t2660 * t33148;
    let t33150 = t33149 * t15483;
    let t33152 = t519 * t9252;
    let t33154 = t1084 * t33152 * t9865;
    (t33144, t33147, t33148, t33149, t33150, t33152, t33154)
}
