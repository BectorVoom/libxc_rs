//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 968/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk968<F: Float>(t1775: F, t4220: F, t2: F, t4129: F, t2681: F, t824: F, t2347: F, t852: F, t3886: F, t2360: F, t1212: F, t2781: F) -> (F, F, F, F, F) {
    let t15028 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1775 * t4220;
    let t15037 = t2 * t4129;
    let t15039 = t2681 * t15037 * t824;
    let t15042 = t852 * t2347;
    let t15043 = t3886 * t824;
    let t15044 = t15042 * t15043;
    let t15047 = t852 * t2360;
    let t15048 = t15047 * t15043;
    let t15051 = t2781 * t1212;
    (t15028, t15039, t15044, t15048, t15051)
}
