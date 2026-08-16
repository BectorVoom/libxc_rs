//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1536/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1536<F: Float>(t23958: F, t993: F, t225: F, t366: F, t20020: F, t4858: F, t1011: F, t140: F, t23877: F, t15823: F, t20029: F, t11710: F, t23899: F, t4892: F) -> (F, F, F, F, F, F, F) {
    let t79862 = t23958 * t993;
    let t79863 = t79862 * t225;
    let t79864 = t79863 * t366;
    let t79874 = t4858 * t20020;
    let t79881 = t1011 * t140 * t23877;
    let t79892 = t15823 * t20029;
    let t79938 = t4892 * t11710 * t23899;
    (t79862, t79863, t79864, t79874, t79881, t79892, t79938)
}
