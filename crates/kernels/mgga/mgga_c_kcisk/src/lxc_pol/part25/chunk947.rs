//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 947/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk947<F: Float>(t16026: F, t7012: F, t16022: F, t3521: F, t7001: F, t16013: F, t7000: F, t11417: F, t708: F, t15999: F, t16009: F, t2364: F, t4604: F, t4652: F, t1648: F, t6714: F) -> (F, F, F, F, F, F, F, F) {
    let t16752 = t7012 * t16026;
    let t16755 = t7012 * t16022;
    let t16759 = 0.14600954814814814815e-2 * t3521 * t7001;
    let t16760 = t7000 * t16013;
    let t16763 = t11417 * t708;
    let t16764 = t16763 * t15999;
    let t16767 = t7000 * t16009;
    let t16771 = t4604 * t2364 * t4652;
    let t16775 = t4604 * t6714 * t1648;
    (t16752, t16755, t16759, t16760, t16764, t16767, t16771, t16775)
}
