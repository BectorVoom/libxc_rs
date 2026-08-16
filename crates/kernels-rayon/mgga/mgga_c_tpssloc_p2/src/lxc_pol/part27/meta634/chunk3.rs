//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2139/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2139(t87165: f64, t81615: f64, t22986: f64, t25236: f64, t2647: f64, t6646: f64, t13381: f64, t1888: f64, t7524: f64, t81612: f64, t81613: f64, t4240: f64, t81865: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87166 = 0.16449340668482264365e-1_f64 * t87165;
    let t87167 = 0.16449340668482264365e-1_f64 * t81615;
    let t87171 = t22986 * t6646 * t25236 * t2647;
    let t87174 = t1888 * t6646 * t13381;
    let t87177 = t81612 * t81613 * t7524;
    let t87183 = t81865 * t4240;
    (t87166, t87167, t87171, t87174, t87177, t87183)
}
