//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 821/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk821(t3209: f64, t830: f64, t2530: f64, t935: f64, t3234: f64, t723: f64) -> (f64, f64, f64) {
    let t28002 = t830 * t3209;
    let t28013 = t935 * t2530;
    let t28023 = t3234 * t723;
    (t28002, t28013, t28023)
}
