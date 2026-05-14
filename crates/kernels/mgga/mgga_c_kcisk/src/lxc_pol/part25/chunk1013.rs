//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1013/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1013<F: Float>(t17914: F, t1948: F, t2586: F, t5335: F, t741: F, t5278: F, t7424: F, t11799: F, t2564: F, t16660: F, t5290: F, t5289: F, t17807: F, t7303: F, t16711: F, t7311: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17915 = t1948 * t17914;
    let t17917 = t2586 * t5335;
    let t17918 = t741 * t17917;
    let t17920 = t5278 * t7424;
    let t17922 = t11799 * t2564;
    let t17924 = t5290 * t16660;
    let t17925 = t5289 * t17924;
    let t17927 = t7303 * t17807;
    let t17928 = t5289 * t17927;
    let t17930 = t7311 * t16711;
    (t17915, t17917, t17918, t17920, t17922, t17924, t17925, t17928, t17930)
}
