//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2089/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2089(t22832: f64, t5234: f64, t1336: f64, t22759: f64, t5252: f64, t836: f64, t5293: f64, t80820: f64, t1831: f64, t80869: f64, t22783: f64, t5314: f64) -> (f64, f64, f64, f64, f64) {
    let t91100 = t5234 * t22832;
    let t91113 = t1336 * t22759 * t836 * t5252;
    let t91114 = 7.0_f64 / 576.0_f64 * t91113;
    let t91120 = t80820 * t5293;
    let t91121 = 7.0_f64 / 1152.0_f64 * t91120;
    let t91135 = t80869 * t1831;
    let t91136 = 7.0_f64 / 288.0_f64 * t91135;
    let t91137 = t22783 * t5314;
    (t91100, t91114, t91121, t91136, t91137)
}
