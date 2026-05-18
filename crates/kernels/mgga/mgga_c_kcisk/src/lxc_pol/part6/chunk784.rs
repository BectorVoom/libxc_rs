//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 784/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk784<F: Float>(t2079: F, t3638: F, t2214: F, t3805: F, t2211: F, t3783: F, t13955: F, t2178: F, t13900: F, t2163: F, t1309: F, t2160: F, t3981: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t19580 = t2079 * t3638;
    let t19788 = t3805 * t2214;
    let t19848 = t2211 * t3783;
    let t19849 = t19848 * sigma0;
    let t19948 = t13955 * t2178;
    let t20127 = t13900 * t2163;
    let t20128 = t1309 * t20127;
    let t20169 = t2160 * t3981;
    (t19580, t19788, t19848, t19849, t19948, t20128, t20169)
}
