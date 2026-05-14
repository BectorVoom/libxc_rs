//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1211/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1211<F: Float>(t3168: F, t4878: F, t13392: F, t4801: F, t1042: F, t11150: F, t3181: F, t15936: F, t4806: F, t11144: F, t11852: F, t3124: F, t4820: F, t1655: F, t697: F, t1011: F) -> (F, F, F, F, F, F, F) {
    let t16190 = t4878 * t3168;
    let t16195 = t4801 * t13392;
    let t16196 = t1042 * t16195;
    let t16199 = t3181 * t11150;
    let t16200 = t16199 * t15936;
    let t16201 = t1042 * t16200;
    let t16204 = t4806 * t13392;
    let t16205 = t1042 * t16204;
    let t16208 = t11852 * t11144;
    let t16209 = t16208 * t15936;
    let t16210 = t1042 * t16209;
    let t16218 = 0.28582678745379824648e-3 * t3124 * t4820;
    let t16219 = t697 * t1655;
    let t16220 = t1011 * t16219;
    (t16190, t16196, t16201, t16205, t16210, t16218, t16220)
}
