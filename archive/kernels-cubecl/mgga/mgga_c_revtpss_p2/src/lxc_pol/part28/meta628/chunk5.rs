//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2264/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2264<F: Float>(t25188: F, t7937: F, t1936: F, t49686: F, t75667: F, t13426: F, t7002: F, t75485: F, t18227: F, t25832: F, t4248: F, t13514: F, t1518: F, t2371: F, t25805: F, t28025: F, t28030: F, t4292: F, t670: F, t6985: F, t92737: F, t97622: F, t97632: F, t98507: F) -> (F, F) {
    let t101486 = t25188 * t7937;
    let t101504 = F::cast_from(2.0_f64) * t49686 * t1936;
    let t101506 = F::cast_from(4.0_f64) * t75667 * t1936;
    let t101508 = F::cast_from(4.0_f64) * t13426 * t7002;
    let t101510 = F::cast_from(2.0_f64) * t75485 * t1936;
    let t101512 = F::cast_from(4.0_f64) * t18227 * t7002;
    let t101514 = F::cast_from(2.0_f64) * t4248 * t25832;
    let t101515 = F::cast_from(2.0_f64) * t13514 * t6985 + F::cast_from(2.0_f64) * t1518 * t92737 + F::cast_from(4.0_f64) * t1518 * t97632 + F::cast_from(2.0_f64) * t1518 * t98507 + F::cast_from(2.0_f64) * t2371 * t28030 + F::cast_from(4.0_f64) * t25805 * t4292 + F::cast_from(4.0_f64) * t28025 * t4292 + F::cast_from(4.0_f64) * t670 * t97622 + t101504 + t101506 + t101508 + t101510 + t101512 + t101514;
    (t101486, t101515)
}
