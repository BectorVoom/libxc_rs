//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 564/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk564(t741: f64, t963: f64, t1859: f64, t897: f64, t1862: f64, t2483: f64, t76: f64) -> (f64, f64, f64, f64) {
    let t2741 = t963 * t741;
    let t2743 = t1859 * t897;
    let t2744 = t2743 * t1862;
    let t2747 = t2483 * t76;
    (t2741, t2743, t2744, t2747)
}
