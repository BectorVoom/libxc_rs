//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 773/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk773<F: Float>(t15128: F, t7672: F, t1234: F, t7584: F, t7641: F, t33811: F, t7512: F, t1091: F, t33821: F, t33822: F, t33820: F, t1212: F) -> (F, F, F, F, F, F, F) {
    let t35817 = t15128 * t7672;
    let t35819 = t7584 * t1234;
    let t35820 = t7641 * t35819;
    let t35822 = t33811 * t7512 * t35820;
    let t35825 = t33821 * t33822 * t1091;
    let t35826 = t33820 * t35825;
    let t35828 = t7584 * t1212;
    (t35817, t35819, t35820, t35822, t35825, t35826, t35828)
}
