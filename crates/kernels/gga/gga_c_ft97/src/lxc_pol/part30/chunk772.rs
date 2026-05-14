//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 772/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk772<F: Float>(t245: F, t35306: F, t35785: F, t21: F, t5: F, t7565: F, t920: F, t33983: F, t6970: F, t193: F, t33966: F, t28985: F, t6222: F, t1091: F, t34001: F, t2665: F, t10248: F, t34006: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t246 = 10000000.0 <= t245;
    let t35786 = t35306 + t35785;
    let t35793 = piecewise3(t246, 0.0, t5 * t35786 * t21 / 4.0 + t5 * t7565 * t920 / 4.0);
    let t35794 = t33983 * t6970;
    let t35795 = t193 * t35794;
    let t35798 = t33966 * t6970;
    let t35799 = t193 * t35798;
    let t35801 = t6222 * t28985;
    let t35802 = t193 * t35801;
    let t35809 = t34001 * t1091;
    let t35810 = t2665 * t35809;
    let t35814 = t10248 * t34006 * t1091;
    (t35786, t35793, t35794, t35795, t35798, t35799, t35801, t35802, t35810, t35814)
}
