//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 936/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk936<F: Float>(t10856: F, t2158: F, t2111: F, t2164: F, t6190: F, t1050: F, t120: F, t6239: F, t269: F, t787: F) -> (F, F, F, F) {
    let t10857 = t10856 * t2158;
    let t10863 = t2111 * t6190 * t2164;
    let t10866 = t120 * t6239 * t1050;
    let t10868 = t787 * t269;
    (t10857, t10863, t10866, t10868)
}
