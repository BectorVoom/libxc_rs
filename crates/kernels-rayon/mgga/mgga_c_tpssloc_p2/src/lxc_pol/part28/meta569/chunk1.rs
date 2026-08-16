//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1849/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1849(t23030: f64, t25258: f64, t13384: f64, t22986: f64, t2647: f64, t6646: f64, t22893: f64, t23164: f64, t25306: f64, t25236: f64, t13381: f64, t1888: f64) -> (f64, f64, f64, f64, f64) {
    let t87155 = t23030 * t25258;
    let t87159 = t22986 * t6646 * t13384 * t2647;
    let t87165 = t23164 * t22893 * t25306;
    let t87171 = t22986 * t6646 * t25236 * t2647;
    let t87174 = t1888 * t6646 * t13381;
    (t87155, t87159, t87165, t87171, t87174)
}
