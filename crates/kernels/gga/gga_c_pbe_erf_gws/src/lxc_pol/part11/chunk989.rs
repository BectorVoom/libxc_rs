//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 989/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk989<F: Float>(t17646: F, t3390: F, t3465: F, t639: F, t1640: F, t47377: F, t5401: F, t1661: F, t47391: F, t5294: F, t587: F, t10843: F, t3504: F, t12564: F, t2612: F, t11032: F, t3500: F) -> (F, F, F, F, F, F) {
    let t47878 = 16.0 / 9.0 * t639 * t17646 * t3465 * t3390;
    let t47882 = 16.0 / 3.0 * t639 * t1640 * t5401 * t47377;
    let t47886 = 16.0 / 3.0 * t587 * t1661 * t5294 * t47391;
    let t47888 = 32.0 / 15.0 * t10843 * t3504;
    let t47890 = 32.0 / 9.0 * t2612 * t12564;
    let t47892 = 16.0 / 15.0 * t11032 * t3500;
    (t47878, t47882, t47886, t47888, t47890, t47892)
}
