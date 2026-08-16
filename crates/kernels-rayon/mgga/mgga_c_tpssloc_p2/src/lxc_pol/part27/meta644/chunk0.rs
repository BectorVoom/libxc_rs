//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2198/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2198(t25796: f64, t4547: f64, t25425: f64, t82431: f64, t25816: f64, t3173: f64, t883: f64, t25443: f64, t1049: f64, t7577: f64, t7557: f64, t82573: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t88058 = t4547 * t25796;
    let t88069 = 0.36554090374405031922e-2_f64 * t82431 * t25425;
    let t88075 = 0.18277045187202515961e-2_f64 * t82431 * t25816;
    let t88076 = t3173 * t883;
    let t88083 = 0.18277045187202515961e-2_f64 * t82431 * t25443;
    let t88089 = t7577 * t1049;
    let t88096 = 0.14621636149762012769e-1_f64 * t82573 * t7557;
    (t88058, t88069, t88075, t88076, t88083, t88089, t88096)
}
