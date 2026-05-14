//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 939/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk939<F: Float>(t16609: F, t5192: F, t6674: F, t1333: F, t6724: F, t6946: F, t2441: F, t4803: F) -> (F, F, F, F, F, F) {
    let t16610 = t5192 * t16609;
    let t16611 = t6674 * t16610;
    let t16613 = t1333 * t6724;
    let t16614 = 0.88437037037037037034e-2 * t16613;
    let t16615 = t1333 * t6946;
    let t16616 = 0.33163888888888888888e-2 * t16615;
    let t16617 = t2441 * t4803;
    (t16611, t16613, t16614, t16615, t16616, t16617)
}
