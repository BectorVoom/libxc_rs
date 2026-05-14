//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 827/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk827<F: Float>(t322: F, t9675: F, t2941: F, t833: F, t1299: F, t2944: F, t829: F, t1013: F, t2394: F, t1300: F, t2397: F, t327: F, t6693: F, t834: F) -> (F, F, F, F, F, F) {
    let t324 = 0.0 < t322;
    let t332 = 0.25e1 < t322;
    let t9676 = piecewise3(t324, 0.0, t9675);
    let t9679 = t2941 * t833;
    let t9684 = t2944 * t1299;
    let t9687 = t2944 * t829;
    let t9690 = t1013 * t2394;
    let t9693 = t2941 * t829;
    let t9698 = -0.64e0 * t9676 * t327 - 0.128e1 * t9679 * t829 - 0.256e1 * t2397 * t2394 - 0.384e1 * t9684 * t829 - 0.384e1 * t6693 * t9687 - 0.256e1 * t1300 * t9690 - 0.128e1 * t1300 * t9693 - 0.64e0 * t834 * t9676;
    let t9707 = piecewise3(t332, 0.0, t9675);
    (t9676, t9687, t9690, t9693, t9698, t9707)
}
