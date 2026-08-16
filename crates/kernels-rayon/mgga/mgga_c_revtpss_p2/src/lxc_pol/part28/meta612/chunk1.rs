//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2138/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2138(t28189: f64, t7235: f64, t2014: f64, t7900: f64, t94358: f64, t10416: f64, t13435: f64, t7746: f64, t98522: f64, t98525: f64, t98528: f64, t98530: f64, t98532: f64, t98534: f64, t98537: f64, t98539: f64, t98541: f64, t98544: f64, t98546: f64, t98549: f64, t98553: f64, t98555: f64, t98557: f64) -> f64 {
    let t98559 = 2.0_f64 * t7235 * t28189;
    let t98562 = 3.0_f64 * t2014 * t94358 * t7900;
    let t98563 = -2.0_f64 * t10416 * t7746 - 4.0_f64 * t13435 * t7746 - t98522 + t98525 - t98528 + t98530 - t98532 - t98534 - t98537 - t98539 + t98541 - t98544 + t98546 + t98549 + t98553 + t98555 + t98557 - t98559 + t98562;
    t98563
}
