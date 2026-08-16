//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1097/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1097<F: Float>(t5036: F, t990: F, t2776: F, t9081: F, t948: F, t4977: F, t975: F, t2786: F, t3949: F, t9095: F, t1464: F, t3987: F) -> (F, F, F, F, F, F) {
    let t15139 = t5036 * t990;
    let t15140 = t2776 * t15139;
    let t15143 = t9081 * t948;
    let t15147 = t975 * t4977;
    let t15151 = t2786 * t3949;
    let t15155 = t9095 * t948;
    let t15162 = t3987 * t1464;
    (t15140, t15143, t15147, t15151, t15155, t15162)
}
