//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1078/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1078(t1245: f64, t124891: f64, t33515: f64, t33426: f64, t372: f64, t3736: f64, t8937: f64, t96881: f64, t1243: f64, t45551: f64, t247: f64, t44545: f64, t494: f64, t8926: f64) -> (f64, f64, f64, f64, f64) {
    let t124893 = t33515 * t124891 * t1245;
    let t124898 = t372 * t33426;
    let t124903 = t8937 * t96881 * t3736;
    let t124915 = t45551 * t1243;
    let t124927 = 0.62743259463289926663e-4_f64 * t8926 * t247 * t44545 * t494;
    (t124893, t124898, t124903, t124915, t124927)
}
