//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 577/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk577<F: Float>(t25875: F, t5674: F, t1316: F, t3051: F, t1564: F, t3052: F, t5675: F, t473: F, t942: F, t1871: F, t22952: F, t432: F, t965: F, t23008: F, t92: F, t452: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25876 = t5674 * t25875;
    let t25878 = t1316 * t3051;
    let t25880 = t1564 * t5675 * t3052;
    let t25881 = t25878 * t25880;
    let t25883 = t942 * t473;
    let t25885 = t1871 * t5675 * t25883;
    let t25886 = t22952 * t25885;
    let t25888 = t965 * t432;
    let t25890 = t1871 * t5675 * t25888;
    let t25891 = t22952 * t25890;
    let t25893 = t23008 * t92;
    let t25894 = t965 * t473;
    let t25896 = t452 * t5675 * t25894;
    (t25876, t25878, t25881, t25883, t25886, t25888, t25891, t25893, t25894, t25896)
}
