//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1317/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1317<F: Float>(t34674: F, t34677: F, t17182: F, t34217: F, t9664: F, t17010: F, t1772: F, t648: F, t32989: F, t7218: F, t34200: F, t5074: F, t34203: F, t34206: F, t1944: F, t2454: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t116063 = t34674 / 8.0;
    let t116064 = t34677 / 8.0;
    let t116116 = t17182 * t34217;
    let t116118 = 0.69444444444444444446e-2 * t9664 * t116116;
    let t116120 = t17010 * t648 * t1772;
    let t116123 = t32989 * t7218;
    let t116126 = t5074 * t34200;
    let t116127 = 0.22109259259259259258e-2 * t116126;
    let t116129 = t5074 * t34203;
    let t116130 = 0.14739506172839506172e-2 * t116129;
    let t116133 = t5074 * t34206;
    let t116137 = t1944 * t2454;
    (t116063, t116064, t116116, t116118, t116120, t116123, t116126, t116127, t116129, t116130, t116133, t116137)
}
