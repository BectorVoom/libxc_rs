//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1685/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1685(t3011: f64, t3014: f64, t88351: f64, t981: f64, t6392: f64, t6244: f64, t6258: f64, t42013: f64, t63453: f64, t63459: f64, t63464: f64, t77499: f64, t77559: f64, t77561: f64, t88085: f64, t88089: f64, t88093: f64, t88097: f64) -> (f64, f64, f64, f64) {
    let t88607 = 0.51947577317044391277e2_f64 * t981 * t3011 * t88351 * t3014;
    let t88628 = t6392 * t6392;
    let t88646 = t6244 * t6258;
    let t88660 = 0.22222222222222222222e-1_f64 * t77559 - 0.66666666666666666668e-1_f64 * t77561 + 0.12345679012345679012e-1_f64 * t77499 - 0.14814814814814814815e-1_f64 * t63453 + 0.44444444444444444445e-1_f64 * t63459 + t42013 + 0.2e0_f64 * t88085 - 0.3e0_f64 * t88089 + 0.50000000000000000001e-1_f64 * t88093 + 0.66666666666666666668e-1_f64 * t88097 - 0.22222222222222222222e-1_f64 * t63464;
    (t88607, t88628, t88646, t88660)
}
