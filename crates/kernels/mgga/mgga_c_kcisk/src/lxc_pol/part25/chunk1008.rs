//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1008/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1008<F: Float>(t2567: F, t5302: F, t734: F, t16653: F, t7430: F, t7429: F, t11226: F, t5320: F, t16716: F, t1941: F, t7401: F, t16648: F, t1894: F, t6689: F, t5290: F, t7315: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17840 = t2567 * t5302;
    let t17841 = t734 * t17840;
    let t17843 = t7430 * t16653;
    let t17844 = t7429 * t17843;
    let t17846 = t11226 * t5320;
    let t17847 = t7430 * t16716;
    let t17848 = t17846 * t17847;
    let t17850 = t7401 * t1941;
    let t17852 = t7430 * t16648;
    let t17853 = t7429 * t17852;
    let t17855 = t6689 * t1894;
    let t17856 = t5290 * t17855;
    let t17857 = t7315 * t17856;
    (t17841, t17843, t17844, t17847, t17848, t17850, t17852, t17853, t17855, t17857)
}
