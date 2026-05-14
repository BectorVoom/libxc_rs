//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 701/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk701<F: Float>(t2326: F, t3394: F, t6514: F, t9074: F, t30204: F, t31769: F, t10177: F, t19531: F, t883: F, t12797: F, t1358: F, t31591: F, t4261: F, t2321: F, t34600: F, t12830: F, t29874: F) -> (F, F, F, F, F, F, F) {
    let t42644 = t9074 * t6514 * t3394 * t2326;
    let t42647 = t9074 * t30204 * t31769;
    let t42651 = t9074 * t19531 * t883 * t10177;
    let t42673 = t1358 * t12797;
    let t42717 = t9074 * t4261 * t31591;
    let t42721 = t9074 * t34600 * t2321;
    let t42820 = t29874 * t12830;
    (t42644, t42647, t42651, t42673, t42717, t42721, t42820)
}
