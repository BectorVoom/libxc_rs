//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 862/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk862<F: Float>(t193: F, t35801: F, t1091: F, t34001: F, t2665: F, t10248: F, t34006: F, t15128: F, t7672: F, t1234: F, t7584: F, t7641: F) -> (F, F, F, F, F, F) {
    let t35802 = t193 * t35801;
    let t35809 = t34001 * t1091;
    let t35810 = t2665 * t35809;
    let t35814 = t10248 * t34006 * t1091;
    let t35817 = t15128 * t7672;
    let t35819 = t7584 * t1234;
    let t35820 = t7641 * t35819;
    (t35802, t35810, t35814, t35817, t35819, t35820)
}
