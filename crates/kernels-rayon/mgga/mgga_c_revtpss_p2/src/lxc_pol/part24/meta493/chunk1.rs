//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1492/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1492(t1437: f64, t2482: f64, t6843: f64, t136: f64, t2457: f64, t3964: f64, t6888: f64, t10073: f64, t22365: f64, t22373: f64, t10069: f64, t22369: f64) -> (f64, f64, f64, f64, f64) {
    let t74892 = t2482 * t1437 * t6843;
    let t74901 = t3964 * t6888 * t136 * t2457;
    let t74945 = t10073 * t22365;
    let t74990 = t10073 * t22373;
    let t74999 = t10069 * t22369;
    (t74892, t74901, t74945, t74990, t74999)
}
