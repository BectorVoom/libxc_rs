//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 672/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk672(t3755: f64, t653: f64, t2211: f64, t442: f64, t128: f64, t818: f64, t2716: f64, t2188: f64, t435: f64, t188: f64, t3: f64, t761: f64, t825: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6194 = t3755 * t653;
    let t6201 = t2211 * t442;
    let t6210 = t128 * t818;
    let t6773 = t2716 * t442;
    let t6791 = t435 * t2188;
    let t6803 = t3 * t188;
    let t6808 = t761 * t825;
    (t6194, t6201, t6210, t6773, t6791, t6803, t6808)
}
