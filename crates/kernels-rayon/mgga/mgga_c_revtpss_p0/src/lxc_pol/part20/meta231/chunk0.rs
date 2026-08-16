//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1025/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1025(t10811: f64, t2751: f64, t2681: f64, t820: f64, t823: f64, t839: f64, t2430: f64, t775: f64, t2477: f64, t828: f64, t222: f64, t9727: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10812 = t10811 * t2751;
    let t10815 = t820 * t823 * t2681;
    let t10816 = t10815 * t839;
    let t10818 = t775 * t2430;
    let t10820 = t2477 * t828 * t10818;
    let t10824 = 455.0_f64 / 1296.0_f64 * t9727 * t222;
    (t10812, t10815, t10816, t10818, t10820, t10824)
}
