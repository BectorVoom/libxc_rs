//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1031/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1031(t233: f64, t240: f64, t27: f64, t119833: f64, t124: f64, t257: f64, t10779: f64, t775: f64, t2684: f64, t8486: f64, t25410: f64, t7063: f64, t8471: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t119835 = t233 * t27 * t240;
    let t119836 = t119833 * t119835;
    let t119837 = t124 * t257;
    let t119839 = t10779 * t119837 * t775;
    let t119840 = t119836 * t119839;
    let t119842 = t8486 * t2684;
    let t119843 = 0.49169913065300780973e-2_f64 * t119842;
    let t119849 = t7063 * t8471 * t25410;
    (t119835, t119836, t119837, t119839, t119840, t119843, t119849)
}
