//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1223/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1223(t3011: f64, t6205: f64, t4733: f64, t981: f64, t15258: f64, t4732: f64, t4719: f64, t4729: f64, t19136: f64, t19143: f64, t19145: f64, t19149: f64, t19152: f64, t19252: f64, t19258: f64, t19315: f64, t19317: f64, t19320: f64, t19323: f64, t19326: f64, t19329: f64, t19333: f64, t19337: f64) -> (f64, f64, f64, f64) {
    let t19467 = t3011 * t6205;
    let t19468 = t19467 * t4733;
    let t19470 = 0.17315859105681463759e2_f64 * t981 * t19468;
    let t19471 = t4732 * t15258;
    let t19473 = 0.34631718211362927518e2_f64 * t981 * t19471;
    let t19475 = 0.11696447245269292414e1_f64 * t4719 * t4729;
    let t19476 = t19136 + t19143 - t19145 + t19149 + t19152 + t19337 + t19252 + t19258 - t19315 + t19317 + t19320 - t19323 - t19326 - t19329 + t19333 - t19470 - t19473 - t19475;
    (t19470, t19473, t19475, t19476)
}
