//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2260/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2260(t13426: f64, t1453: f64, t2320: f64, t27076: f64, t29337: f64, t29437: f64, t4248: f64, t649: f64, t7591: f64, t8233: f64, t98525: f64, t98528: f64, t98530: f64, t98532: f64, t98534: f64, t98537: f64, t98539: f64, t98541: f64, t98544: f64, t98546: f64, t98549: f64, t98553: f64, t98555: f64, t98557: f64) -> f64 {
    let t105724 = -4.0_f64 * t13426 * t7591 + 2.0_f64 * t1453 * t29437 - t2320 * t8233 - 4.0_f64 * t27076 * t4248 - 2.0_f64 * t29337 * t649 + t98525 - t98528 + t98530 - t98532 - t98534 - t98537 - t98539 + t98541 - t98544 + t98546 + t98549 + t98553 + t98555 + t98557;
    t105724
}
