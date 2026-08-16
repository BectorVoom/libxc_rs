//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1121/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1121(t198: f64, t8034: f64, t2718: f64, t7997: f64, t670: f64, t7968: f64, t8107: f64, t9593: f64, t7983: f64, t8151: f64, t84: f64, t8440: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t110165 = t198 * t8034;
    let t110687 = t2718 * t7997;
    let t111018 = t7968 * t670;
    let t111176 = t8107 * t9593;
    let t111371 = t670 * t7983;
    let t111734 = t8151 * t670;
    let t119457 = t8440 * t84;
    (t110165, t110687, t111018, t111176, t111371, t111734, t119457)
}
