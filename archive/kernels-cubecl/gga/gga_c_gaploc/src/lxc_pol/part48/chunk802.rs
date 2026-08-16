//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 802/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk802<F: Float>(t12797: F, t1358: F, t31591: F, t4261: F, t9074: F, t2321: F, t34600: F, t12820: F, t484: F, t12770: F, t2312: F, t10590: F, t882: F) -> (F, F, F, F, F, F) {
    let t42673 = t1358 * t12797;
    let t42717 = t9074 * t4261 * t31591;
    let t42721 = t9074 * t34600 * t2321;
    let t42726 = t484 * t12820;
    let t42745 = t2312 * t12770;
    let t42748 = t882 * t10590 * t2321;
    (t42673, t42717, t42721, t42726, t42745, t42748)
}
