//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 868/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk868<F: Float>(t1305: F, t4155: F, t4001: F, t1294: F, t3981: F, t3993: F, t1301: F, t13614: F, t397: F, t403: F, t396: F, t1390: F, t301: F, t1310: F, t1311: F, t164: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13851 = t4155 * t1305;
    let t13859 = t4001 * t1305;
    let t13861 = t1294 * t3981;
    let t13866 = t3993 * t1305;
    let t13868 = t1301 * t3981;
    let t13871 = t397 * t13614 * t403;
    let t13873 = 0.19989765240197019125e-1 * t396 * t13871;
    let t13893 = 1.0 / t301 / t1390;
    let t13894 = t1310 * t13893;
    let t13900 = t164 * t1311;
    (t13851, t13859, t13861, t13866, t13868, t13873, t13893, t13894, t13900)
}
