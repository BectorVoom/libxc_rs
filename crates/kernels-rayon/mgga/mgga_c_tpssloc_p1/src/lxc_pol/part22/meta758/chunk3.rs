//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2547/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2547(t71343: f64, t71396: f64, t71428: f64, t71440: f64, t71467: f64, t71494: f64, t71515: f64, t71527: f64, t51402: f64, t6024: f64, t21961: f64, t44162: f64) -> (f64, f64, f64) {
    let t71530 = t71343 + t71396 + t71428 + t71440 + t71467 + t71494 + t71515 + t71527;
    let t71543 = 0.48245938496077605201e2_f64 * t51402 * t6024;
    let t71545 = 0.96491876992155210402e2_f64 * t44162 * t21961;
    (t71530, t71543, t71545)
}
