//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 967/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk967<F: Float>(t531: F, t8107: F, t116: F, t7983: F, t1450: F, t6816: F, t6836: F, t196: F, t197: F, t6773: F, t5920: F, t94: F, t21663: F, t38: F, t5868: F, t76: F) -> (F, F, F, F, F, F, F, F) {
    let t28938 = t531 * t8107;
    let t28986 = t116 * t7983;
    let t29494 = t1450 * t6816;
    let t29498 = t1450 * t6836;
    let t29506 = t6773 * t196 * t197;
    let t29508 = t94 * t5920;
    let t29513 = t21663 * t38;
    let t29532 = t76 * t5868;
    (t28938, t28986, t29494, t29498, t29506, t29508, t29513, t29532)
}
