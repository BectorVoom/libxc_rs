//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 971/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk971<F: Float>(t35972: F, t824: F, t1486: F, t193: F, t2781: F, t4129: F, t7611: F, t35846: F, t681: F, t35838: F, t6308: F, t1212: F, t33953: F, t35973: F, t10248: F, t18: F, t3281: F, t33961: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t152856 = t35972 * t824;
    let t152859 = t1486 * t193 * t2781 * t152856;
    let t152861 = t7611 * t4129;
    let t152864 = t1486 * t193 * t2781 * t152861;
    let t152867 = t1486 * t681 * t35846;
    let t152870 = t6308 * t681 * t35838;
    let t152872 = t33953 * t1212;
    let t152875 = t1486 * t193 * t2781 * t152872;
    let t152878 = t1486 * t681 * t35973;
    let t152882 = t3281 * t10248 * t33961 * t18;
    (t152856, t152859, t152861, t152864, t152867, t152870, t152872, t152875, t152878, t152882)
}
