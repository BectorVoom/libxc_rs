//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3840/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3840(t22076: f64, t9962: f64, t6861: f64, t9994: f64, t1353: f64, t5658: f64, t1398: f64, t125: f64, t22252: f64, t124: f64, t6843: f64, t3938: f64, t9816: f64, t9818: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t73818 = t9962 * t22076;
    let t73820 = t6861 * t9994;
    let t73837 = t1353 * t5658;
    let t73842 = t6861 * t1398;
    let t73847 = t125 * t22252;
    let t73856 = t124 * t6843;
    let t73859 = t9816 * t9818 * t73856 * t3938;
    (t73818, t73820, t73837, t73842, t73847, t73856, t73859)
}
