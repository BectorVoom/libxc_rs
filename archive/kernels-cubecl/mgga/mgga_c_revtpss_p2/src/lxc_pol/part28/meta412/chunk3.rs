//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1557/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1557<F: Float>(t15234: F, t973: F, t2962: F, t4673: F, t11452: F, t1621: F, t2944: F, t4708: F, t972: F, t1634: F, t3006: F, t2988: F, t4711: F) -> (F, F, F, F, F, F) {
    let t15235 = t15234 * t973;
    let t15238 = t4673 * t2962;
    let t15241 = t1621 * t11452;
    let t15242 = t15241 * t2944;
    let t15249 = t4708 * t972;
    let t15252 = t1634 * t3006;
    let t15255 = t4711 * t2988;
    (t15235, t15238, t15242, t15249, t15252, t15255)
}
