//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1632/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1632(t1261: f64, t12944: f64, t3172: f64, t12932: f64, t3711: f64, t221: f64, t461: f64, t462: f64, t624: f64, t1250: f64, t606: f64, t1248: f64, t2258: f64) -> (f64, f64, f64, f64) {
    let t44789 = t1261 * t3172 * t12944;
    let t44792 = t3711 * t3172 * t12932;
    let t44797 = 5.0_f64 / 486.0_f64 * t461 * t221 * t624 * t462;
    let t44799 = t1250 * t606;
    let t44800 = t2258 * t1248 * t44799;
    (t44789, t44792, t44797, t44800)
}
