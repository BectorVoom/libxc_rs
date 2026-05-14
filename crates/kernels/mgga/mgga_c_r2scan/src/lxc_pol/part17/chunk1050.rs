//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1050/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1050<F: Float>(t10760: F, t29783: F, t6093: F, t3591: F, t39739: F, t2147: F, t30304: F, t3178: F, t545: F, t3300: F, t3290: F, t9302: F, t12486: F, t24039: F, t10856: F, t9236: F) -> (F, F, F, F, F, F, F) {
    let t43670 = t6093 * t10760 * t29783;
    let t43672 = t39739 * t3591;
    let t43677 = t2147 * t10760 * t30304;
    let t43681 = t545 * t3178;
    let t43682 = t43681 * t3300;
    let t43688 = t3290 * t9302;
    let t43690 = t24039 * t12486;
    let t43692 = t10856 * t9236;
    (t43670, t43672, t43677, t43682, t43688, t43690, t43692)
}
