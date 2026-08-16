//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1358/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1358(t1484: f64, t23097: f64, t5617: f64, t815: f64, t1509: f64, t20986: f64, t2628: f64, t6605: f64, t20969: f64, t6614: f64, t1512: f64, t98684: f64) -> (f64, f64, f64, f64) {
    let t105329 = t23097 * t815 * t5617 * t1484;
    let t105333 = t6605 * t2628 * t20986 * t1509;
    let t105335 = t6614 * t20969;
    let t105337 = t98684 * t1512;
    (t105329, t105333, t105335, t105337)
}
