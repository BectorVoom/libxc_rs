//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 420/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk420(t1474: f64, t172: f64, t763: f64, t1471: f64, t706: f64, t67: f64, t758: f64, t1516: f64, t2697: f64, t1520: f64, t225: f64) -> (f64, f64, f64, f64, f64) {
    let t4199 = t1474 * t172;
    let t4200 = t4199 * t763;
    let t4205 = t706 * t1471;
    let t4211 = t1474 * t67;
    let t4212 = t4211 * t758;
    let t4253 = t2697 * t1516;
    let t4268 = t1520 * t225;
    (t4200, t4205, t4212, t4253, t4268)
}
