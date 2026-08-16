//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2914/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2914(t11294: f64, t23565: f64, t19128: f64, t4590: f64, t52219: f64, t6145: f64, t23467: f64, t41883: f64, t23547: f64, t2869: f64, t11385: f64, t15396: f64, t6141: f64, t934: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t77639 = 6.0_f64 * t11294 * t23565;
    let t77641 = 3.0_f64 * t4590 * t19128;
    let t77643 = 0.48245938496077605201e2_f64 * t52219 * t6145;
    let t77645 = 0.96491876992155210402e2_f64 * t41883 * t23467;
    let t77647 = 1.0_f64 * t2869 * t23547;
    let t77657 = 0.1551780387578202009e4_f64 * t11385 * t6141 * t15396 * t934;
    (t77639, t77641, t77643, t77645, t77647, t77657)
}
