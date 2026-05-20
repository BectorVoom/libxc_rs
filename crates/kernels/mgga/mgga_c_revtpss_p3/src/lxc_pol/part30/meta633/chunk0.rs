//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2201/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2201<F: Float>(t25188: F, t7937: F, t1936: F, t49686: F, t75667: F, t13426: F, t7002: F, t75485: F, t18227: F, t25832: F, t4248: F, t98484: F) -> (F, F, F, F, F, F, F, F) {
    let t101486 = t25188 * t7937;
    let t101504 = F::new(2.0) * t49686 * t1936;
    let t101506 = F::new(4.0) * t75667 * t1936;
    let t101508 = F::new(4.0) * t13426 * t7002;
    let t101510 = F::new(2.0) * t75485 * t1936;
    let t101512 = F::new(4.0) * t18227 * t7002;
    let t101514 = F::new(2.0) * t4248 * t25832;
    let t101517 = F::new(2.0) * t98484 * t1936;
    (t101486, t101504, t101506, t101508, t101510, t101512, t101514, t101517)
}
