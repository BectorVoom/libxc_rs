//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1023/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1023(t103: f64, t15513: f64, t786: f64, t7877: f64, t1180: f64, t15805: f64, t2206: f64, t2394: f64, t2211: f64, t2254: f64, t102: f64, t327: f64, t959: f64) -> (f64, f64, f64, f64, f64) {
    let t18317 = t15513 * t786 * t7877 * t103;
    let t18331 = t15805 * t1180;
    let t18551 = t2394 * t2206;
    let t18553 = t2211 * t2254;
    let t18639 = t102 * t327 * t959;
    (t18317, t18331, t18551, t18553, t18639)
}
