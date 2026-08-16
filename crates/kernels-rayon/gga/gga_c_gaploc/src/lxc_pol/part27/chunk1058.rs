//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1058/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1058(t6455: f64, t900: f64, t1423: f64, t2317: f64, t501: f64, t6551: f64, t2530: f64, t723: f64) -> (f64, f64, f64, f64) {
    let t21414 = t900 * t6455;
    let t21417 = t1423 * t2317;
    let t21438 = t6551 * t501;
    let t21446 = t2530 * t723;
    (t21414, t21417, t21438, t21446)
}
