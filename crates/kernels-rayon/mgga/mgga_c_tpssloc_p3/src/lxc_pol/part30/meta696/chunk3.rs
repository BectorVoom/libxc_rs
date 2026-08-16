//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2230/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2230(t1484: f64, t6552: f64, t6637: f64, t87586: f64, t1509: f64, t7510: f64, t1888: f64, t232: f64, t58166: f64, t6646: f64, t16815: f64, t22986: f64, t2647: f64) -> (f64, f64, f64, f64) {
    let t98520 = t6552 * t6637 * t87586 * t1484;
    let t98524 = t7510 * t1509;
    let t98530 = t1888 * t6646 * t58166 * t232;
    let t98534 = t22986 * t6646 * t16815 * t2647;
    (t98520, t98524, t98530, t98534)
}
