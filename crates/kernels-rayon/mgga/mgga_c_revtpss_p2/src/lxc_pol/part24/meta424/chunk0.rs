//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1373/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1373(t12627: f64, t1284: f64, t3624: f64, t3617: f64, t675: f64, t1263: f64, t215: f64, t1121: f64, t13045: f64, t221: f64, t461: f64, t462: f64, t624: f64) -> (f64, f64, f64, f64, f64) {
    let t44609 = t12627 * t1284 * t3624;
    let t44693 = t675 * t3617;
    let t44701 = t215 * t1263;
    let t44737 = t13045 * t1121;
    let t44797 = 5.0_f64 / 486.0_f64 * t461 * t221 * t624 * t462;
    (t44609, t44693, t44701, t44737, t44797)
}
