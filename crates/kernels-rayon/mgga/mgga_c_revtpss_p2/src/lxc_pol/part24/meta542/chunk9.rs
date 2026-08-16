//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1602/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1602(t5: f64, t87225: f64, t117: f64, t5920: f64, t190: f64, t706: f64, t87126: f64, t76892: f64, t23221: f64, t4311: f64, t1522: f64, t77054: f64, t49866: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t87226 = piecewise3(t8, 0.0_f64, t87225);
    let t87227 = t87226 * t117;
    let t87237 = t5920 * t5920;
    let t87262 = 4.0_f64 * t706 * t190 * t87126;
    let t87263 = 144.0_f64 * t76892;
    let t87265 = 16.0_f64 * t4311 * t23221;
    let t87267 = 16.0_f64 * t77054 * t1522;
    let t87268 = 0.4101607543286562663e4_f64 * t49866;
    (t87227, t87237, t87262, t87263, t87265, t87267, t87268)
}
