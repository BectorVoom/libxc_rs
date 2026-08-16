//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1639/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1639(t87302: f64, t87316: f64, t87931: f64, t87942: f64, t87951: f64, t87952: f64, t87966: f64, t87987: f64, t6206: f64, t6226: f64, t981: f64, t19133: f64, t19303: f64) -> (f64, f64, f64) {
    let t87990 = t87302 + t87316 + t87931 + t87942 + t87951 + t87952 + t87966 + t87987;
    let t88004 = 0.21053605041484726346e2_f64 * t981 * t6226 * t6206;
    let t88007 = 0.62337092780453269531e3_f64 * t981 * t19133 * t19303;
    (t87990, t88004, t88007)
}
