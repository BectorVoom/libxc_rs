//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1172/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1172<F: Float>(t1591: F, t1596: F, t32440: F, t6204: F, t3579: F, t9537: F, t1312: F, t1311: F, t1588: F, t1310: F) -> (F, F, F, F, F, F, F) {
    let t32441 = t1596 * t1591;
    let t32442 = t32440 * t32441;
    let t32443 = t6204 * t32442;
    let t32446 = t9537 * t3579;
    let t32447 = t1312 * t32446;
    let t32457 = t1311 * t1588;
    let t32458 = t1310 * t32457;
    (t32441, t32442, t32443, t32446, t32447, t32457, t32458)
}
