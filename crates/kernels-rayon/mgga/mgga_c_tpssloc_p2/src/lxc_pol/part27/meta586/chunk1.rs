//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2042/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2042(t22641: f64, t2588: f64, t225: f64, t814: f64, t6648: f64, t23021: f64, t6547: f64, t23155: f64, t23168: f64, t22893: f64, t23158: f64, t23164: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81612 = t22641 * t2588;
    let t81613 = t225 * t814;
    let t81615 = t81612 * t81613 * t6648;
    let t81617 = t6547 * t23021;
    let t81623 = t23168 * t23155;
    let t81630 = t23164 * t22893 * t23158;
    (t81612, t81613, t81615, t81617, t81623, t81630)
}
