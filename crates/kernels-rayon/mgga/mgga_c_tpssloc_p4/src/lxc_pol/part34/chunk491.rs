//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 491/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk491(t4211: f64, t758: f64, t228: f64, t68: f64, t1484: f64, t845: f64, t1516: f64, t2697: f64, t1520: f64, t225: f64) -> (f64, f64, f64, f64, f64) {
    let t4212 = t4211 * t758;
    let t4225 = t228 * t68;
    let t4226 = t845 * t1484;
    let t4253 = t2697 * t1516;
    let t4268 = t1520 * t225;
    (t4212, t4225, t4226, t4253, t4268)
}
