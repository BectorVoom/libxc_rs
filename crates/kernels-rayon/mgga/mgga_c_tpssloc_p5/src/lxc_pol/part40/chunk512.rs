//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 512/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk512(t109: f64, t532: f64, t556: f64, t656: f64, t91: f64, t96: f64, t64: f64) -> (f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t1995 = 1.0_f64 / t556 / t532;
    let t2176 = t656 * t91;
    let t2177 = t2176 * t96;
    let t2180 = piecewise3(t110, 0.0_f64, -t64 * t2177 / 8.0_f64);
    (t1995, t2177, t2180)
}
