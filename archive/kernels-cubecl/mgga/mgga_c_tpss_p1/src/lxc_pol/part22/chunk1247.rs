//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1247/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1247<F: Float>(t1395: F, t226: F, t782: F, t1379: F, t818: F, t5570: F, t811: F, t1706: F, t10584: F, t10579: F, t1398: F, t750: F) -> (F, F, F, F, F, F, F) {
    let t19748 = t1395 * t782 * t226;
    let t19762 = t1379 * t818;
    let t19766 = t5570 * t811;
    let t19767 = t1706 * t19766;
    let t19769 = t10584 * t782;
    let t19781 = t10579 * t226;
    let t19809 = t1398 * t750;
    (t19748, t19762, t19766, t19767, t19769, t19781, t19809)
}
