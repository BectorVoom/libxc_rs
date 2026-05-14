//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 908/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk908<F: Float>(t1333: F, t6724: F, t6946: F, t2510: F, t3805: F, t4811: F, t6691: F, t10473: F, t2474: F, t5074: F, t6694: F, t1797: F, t2507: F, t1336: F, t140: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16613 = t1333 * t6724;
    let t16614 = 0.88437037037037037034e-2 * t16613;
    let t16615 = t1333 * t6946;
    let t16616 = 0.33163888888888888888e-2 * t16615;
    let t16640 = t3805 * t2510;
    let t16643 = t4811 * t6691;
    let t16658 = t10473 * t2474;
    let t16672 = t5074 * t6694;
    let t16673 = 0.22109259259259259258e-2 * t16672;
    let t16674 = t1797 * t2507;
    let t16676 = t140 * t1336 * t16674;
    (t16613, t16614, t16615, t16616, t16640, t16643, t16658, t16672, t16673, t16676)
}
