//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 918/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk918<F: Float>(t3016: F, t797: F, t3060: F, t11036: F, t2928: F, t2938: F, t3358: F, t1070: F, t9640: F, t3629: F, t8358: F, t6661: F, t1276: F, t2983: F, t352: F, t12428: F, t3275: F, t3472: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12570 = t797 * t3016;
    let t12574 = t797 * t3060;
    let t12587 = t11036 * t2928;
    let t12589 = t3358 * t2938;
    let t12591 = t9640 * t1070;
    let t12593 = t8358 * t3629;
    let t12595 = t1070 * t2928;
    let t12596 = t6661 * t12595;
    let t12598 = t1070 * t2938;
    let t12599 = t1276 * t12598;
    let t12683 = t352 * t2983;
    let t12720 = t3275 * t3472 * t12428;
    (t12570, t12574, t12587, t12589, t12591, t12593, t12595, t12596, t12598, t12599, t12683, t12720)
}
