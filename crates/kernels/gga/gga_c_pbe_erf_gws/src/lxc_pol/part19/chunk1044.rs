//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1044/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1044<F: Float>(t13919: F, t3764: F, t13917: F, t1161: F, t824: F, t1113: F, t13781: F, t3972: F, t1109: F, t2118: F, t3975: F, t1076: F, t331: F, t1123: F, t850: F, t833: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15134 = t13919 * t3764;
    let t15135 = t13917 * t15134;
    let t15144 = t824 * t1161;
    let t15145 = t1113 * t15144;
    let t15146 = t13781 * t15145;
    let t15147 = t3972 * t15146;
    let t15149 = t2118 * t1109;
    let t15150 = t1113 * t15149;
    let t15151 = t3975 * t15150;
    let t15152 = t3972 * t15151;
    let t15159 = t1076 * t331;
    let t15161 = t850 * t1123 * t15159;
    let t15162 = t15161 * t833;
    (t15134, t15135, t15144, t15146, t15147, t15149, t15151, t15152, t15159, t15161, t15162)
}
