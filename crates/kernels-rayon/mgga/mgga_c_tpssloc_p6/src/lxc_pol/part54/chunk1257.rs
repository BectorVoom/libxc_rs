//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1257/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1257(t6875: f64, t8944: f64, t111: f64, t26966: f64, t2094: f64, t40611: f64, t12461: f64, t7216: f64, t193: f64, t7125: f64, t26739: f64, t2752: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91669 = t6875 * t8944;
    let t92090 = t26966 * t111;
    let t92169 = t2094 * t40611;
    let t92200 = t7216 * t12461;
    let t92271 = t193 * t7125;
    let t92276 = t26739 * t2752;
    (t91669, t92090, t92169, t92200, t92271, t92276)
}
