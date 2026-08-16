//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 931/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk931<F: Float>(t10740: F, t254: F, t120: F, t2176: F, t531: F, t2233: F, t3290: F, t2222: F, t2225: F, t2186: F, t261: F, t7628: F) -> (F, F, F, F, F, F, F, F) {
    let t10741 = t254 * t10740;
    let t10743 = t120 * t2176;
    let t10744 = t10743 * t531;
    let t10745 = F::cast_from(0.25610080155860322884e0_f64) * t10744;
    let t10746 = t3290 * t2233;
    let t10748 = t120 * t2222;
    let t10749 = t10748 * t2225;
    let t10752 = t261 * t2186;
    let t10753 = t7628 * t10752;
    (t10741, t10743, t10744, t10745, t10746, t10749, t10752, t10753)
}
