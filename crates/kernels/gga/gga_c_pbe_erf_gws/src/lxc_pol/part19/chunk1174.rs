//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1174/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1174<F: Float>(t3324: F, t4120: F, t1105: F, t1167: F, t13919: F, t3764: F, t13917: F, t1161: F, t824: F, t1113: F, t13781: F, t3972: F) -> (F, F, F, F, F, F, F) {
    let t15113 = t4120 * t3324;
    let t15124 = t1105 * t1167;
    let t15134 = t13919 * t3764;
    let t15135 = t13917 * t15134;
    let t15144 = t824 * t1161;
    let t15145 = t1113 * t15144;
    let t15146 = t13781 * t15145;
    let t15147 = t3972 * t15146;
    (t15113, t15124, t15134, t15135, t15144, t15146, t15147)
}
