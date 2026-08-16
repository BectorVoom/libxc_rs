//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2848/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2848(t61090: f64, t39419: f64, t39422: f64, t39429: f64, t39432: f64, t39442: f64, t49877: f64, t76890: f64, t76893: f64, t76932: f64, t76935: f64, t76936: f64, t76937: f64, t76938: f64, t76939: f64, t76940: f64, t76941: f64) -> (f64, f64) {
    let t76942 = 12.0_f64 * t61090;
    let t76943 = t76890 + t76893 + t76932 - t39419 - t39422 + t76935 + t76936 - t76937 - t76938 - t76939 - t39429 - t39432 + t76940 + t76941 + t39442 + t49877 + t76942;
    (t76942, t76943)
}
