//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 114/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk114(t77: f64, t98: f64, t163: f64, t5: f64, t83: f64, t136: f64) -> (f64, f64, f64, f64) {
    let t403 = t77 * t98;
    let t405 = t5 * t163;
    let t406 = t83 * t405;
    let t408 = 1.0_f64 / t136;
    (t403, t405, t406, t408)
}
