//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1218/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1218(t10298: f64, t607: f64, t2242: f64, t2259: f64, t1928: f64, t25099: f64, t25102: f64, t25110: f64, t25114: f64, t25147: f64, t25157: f64, t25159: f64, t6960: f64, t6963: f64, t6974: f64, t92684: f64, t92687: f64, t92690: f64, t92692: f64, t92696: f64, t92699: f64, t92702: f64) -> f64 {
    let t92709 = t10298 * t607;
    let t92711 = t2242 * t2259;
    let t92715 = t6963 * t25147 - 15.0_f64 * t92684 * t25159 - 15.0_f64 * t92687 * t25159 + 35.0_f64 * t92690 * t92692 - 15.0_f64 * t25157 * t92696 + 5.0_f64 / 2.0_f64 * t92699 * t6960 + 5.0_f64 * t92702 * t6960 + 5.0_f64 * t25099 * t25110 + 5.0_f64 / 2.0_f64 * t25099 * t25114 + t92709 * t1928 + t92711 * t1928 + 2.0_f64 * t25102 * t6974;
    t92715
}
