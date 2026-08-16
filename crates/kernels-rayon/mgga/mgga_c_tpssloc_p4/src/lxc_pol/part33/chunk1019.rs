//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1019/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1019(t1070: f64, t193: f64, t21251: f64, t21255: f64, t21263: f64, t21265: f64, t21267: f64, t21270: f64, t21302: f64, t21305: f64, t21317: f64, t21320: f64, t21336: f64, t21591: f64, t21697: f64, t336: f64) -> f64 {
    let t21701 = t1070 * t193 * t21697 * t336 - t21251 + t21255 + t21263 + t21265 + t21267 - t21270 + t21302 + t21305 - t21317 + t21320 - t21336 - t21591;
    t21701
}
