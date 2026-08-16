//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1820/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1820(t27137: f64, t651: f64, t1843: f64, t1932: f64, t2322: f64, t27116: f64, t27118: f64, t27120: f64, t27122: f64, t27125: f64, t27128: f64, t27130: f64, t27132: f64, t27134: f64, t27136: f64, t5517: f64, t6983: f64, t7746: f64) -> f64 {
    let t27139 = 2.0_f64 * t651 * t27137;
    let t27142 = -t1843 * t6983 - t1932 * t5517 - 2.0_f64 * t2322 * t7746 - t27116 - t27118 - t27120 - t27122 - t27125 - t27128 - t27130 - t27132 - t27134 - t27136 - t27139;
    t27142
}
