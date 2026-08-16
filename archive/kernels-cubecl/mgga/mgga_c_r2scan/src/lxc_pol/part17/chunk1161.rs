//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1161/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1161<F: Float>(t11816: F, t39409: F, t3308: F, t37652: F, t8784: F, t10710: F, t10768: F, t29126: F, t10781: F, t8839: F, t10894: F, t3072: F) -> (F, F, F, F, F) {
    let t43105 = t39409 * t11816;
    let t43108 = t37652 * t3308 * t8784;
    let t43111 = t10768 * t10710 * t29126;
    let t43115 = t10781 * t8839;
    let t43117 = t10894 * t3072;
    (t43105, t43108, t43111, t43115, t43117)
}
