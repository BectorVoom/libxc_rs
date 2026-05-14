//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1039/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1039<F: Float>(t20831: F, t20878: F, t416: F, t467: F, t471: F, t415: F, t2110: F, t3929: F, t140: F, t3737: F, t5631: F, t5636: F, t13959: F, t5628: F, t5622: F, t6235: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20879 = t20831 + t20878;
    let t20880 = t416 * t20879;
    let t20881 = t20880 * t467;
    let t20882 = t20881 * t471;
    let t20883 = t415 * t20882;
    let t20886 = t2110 * t3929;
    let t20890 = t140 * t3737 * t5631;
    let t20891 = t20890 * t5636;
    let t20892 = 0.3684876543209876543e-2 * t20891;
    let t20893 = t13959 * t5628;
    let t20895 = t13959 * t5622;
    let t20896 = 0.14739506172839506172e-2 * t20895;
    let t20897 = t13959 * t6235;
    (t20879, t20880, t20883, t20886, t20891, t20892, t20893, t20895, t20896, t20897)
}
