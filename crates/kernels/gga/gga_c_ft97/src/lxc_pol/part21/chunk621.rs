//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 621/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk621<F: Float>(t15593: F, t947: F, t3108: F, t4414: F, t1537: F, t4500: F, t4436: F, t7241: F, t432: F, t28: F, t89: F, t4418: F, t7780: F, t1546: F, t4426: F, t4432: F) -> (F, F, F, F, F, F, F) {
    let t15594 = t15593 * t947;
    let t15596 = t4414 * t3108;
    let t15599 = t1537 * t4500;
    let t15601 = t7241 * t4436;
    let t15602 = t15601 * t432;
    let t15604 = t89 * t28 * t15602;
    let t15606 = t89 * t7780 * t4418;
    let t15609 = t89 * t1546 * t4426;
    let t15612 = t89 * t1546 * t4432;
    (t15594, t15596, t15599, t15604, t15606, t15609, t15612)
}
