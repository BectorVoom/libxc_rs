//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 846/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk846(t1882: f64, t4003: f64, t1873: f64, t9741: f64, t5651: f64, t808: f64, t9736: f64, t136: f64, t550: f64, t124: f64, t5609: f64, t9794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13790 = t1882 * t4003;
    let t13798 = t9741 * t1873;
    let t13800 = t808 * t5651;
    let t13801 = t9736 * t13800;
    let t13846 = t550 * t136;
    let t13848 = t124 * t1882;
    let t13857 = t9794 * t5609;
    (t13790, t13798, t13801, t13846, t13848, t13857)
}
