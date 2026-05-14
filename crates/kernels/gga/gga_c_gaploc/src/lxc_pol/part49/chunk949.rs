//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 949/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk949<F: Float>(t12223: F, t1445: F, t2530: F, t813: F, t13870: F, t2089: F, t2087: F, t723: F, t13865: F, t4614: F, t39145: F, t787: F, t32970: F, t835: F, t1457: F, t2103: F) -> (F, F, F, F, F, F, F) {
    let t47255 = t813 * t1445 * t12223 * t2530;
    let t47257 = t2089 * t13870;
    let t47261 = 0.69017266717057349418e1 * t2087 * t1445 * t47257 * t723;
    let t47263 = t2087 * t4614 * t13865;
    let t47266 = t787 * t39145;
    let t47267 = t47266 * t32970;
    let t47270 = t835 * t13870;
    let t47271 = t47270 * t723;
    let t47274 = 0.71500979903700853338e0 * t2103 * t1457 * t47271;
    (t47255, t47261, t47263, t47267, t47270, t47271, t47274)
}
