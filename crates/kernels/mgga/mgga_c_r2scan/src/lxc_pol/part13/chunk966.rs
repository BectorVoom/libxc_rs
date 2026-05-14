//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 966/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk966<F: Float>(t1591: F, t37754: F, t10768: F, t6214: F, t10710: F, t20294: F, t10697: F, t10780: F, t2127: F, t2214: F, t503: F, t6156: F, t2183: F, t573: F, t10776: F, t10810: F, t2135: F) -> (F, F, F, F, F, F, F, F, F) {
    let t37755 = t1591 * t37754;
    let t37759 = t10768 * t6214;
    let t37762 = t10768 * t10710 * t20294;
    let t37764 = t10697 * t10780;
    let t37765 = t37764 * t2127;
    let t37769 = t503 * t2214;
    let t37770 = t37769 * t6156;
    let t37782 = t2183 * t573;
    let t37788 = t10776 * t10810 * t2135;
    (t37755, t37759, t37762, t37764, t37765, t37769, t37770, t37782, t37788)
}
