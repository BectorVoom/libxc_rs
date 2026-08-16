//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1984/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1984(t1388: f64, t6330: f64, t6463: f64, t1307: f64, t5449: f64, t671: f64, t1851: f64, t1372: f64, t794: f64, t213: f64, t225: f64, t22716: f64, t6908: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t75203 = t6330 * t1388;
    let t75210 = t6463 * t1388;
    let t75214 = t6463 * t1307;
    let t75560 = t5449 * t671;
    let t75795 = t1851 * t671;
    let t80645 = t794 * t1372;
    let t80650 = t213 * t1372 * t225;
    let t80663 = t22716 * t6908;
    (t75203, t75210, t75214, t75560, t75795, t80645, t80650, t80663)
}
