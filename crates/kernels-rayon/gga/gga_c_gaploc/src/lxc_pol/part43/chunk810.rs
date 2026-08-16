//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 810/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk810(t28013: f64, t5641: f64, t883: f64, t9805: f64, t23000: f64, t27997: f64, t2624: f64, t9800: f64, t9829: f64, t2617: f64, t3255: f64, t7803: f64) -> (f64, f64, f64, f64) {
    let t40956 = t9805 * t5641 * t883 * t28013;
    let t40966 = t23000 * t5641 * t883 * t27997;
    let t40969 = t9800 * t2624 * t9829;
    let t40986 = t7803 * t3255 * t2617;
    (t40956, t40966, t40969, t40986)
}
