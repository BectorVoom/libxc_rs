//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2219/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2219<F: Float>(t13392: F, t4801: F, t1042: F, t11150: F, t3181: F, t15936: F, t4806: F, t11144: F, t11852: F) -> (F, F, F, F, F, F, F, F) {
    let t16195 = t4801 * t13392;
    let t16196 = t1042 * t16195;
    let t16199 = t3181 * t11150;
    let t16200 = t16199 * t15936;
    let t16201 = t1042 * t16200;
    let t16204 = t4806 * t13392;
    let t16205 = t1042 * t16204;
    let t16208 = t11852 * t11144;
    (t16195, t16196, t16199, t16200, t16201, t16204, t16205, t16208)
}
