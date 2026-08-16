//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 967/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk967(t1379: f64, t9709: f64, t2689: f64, t3952: f64, t1413: f64, t3889: f64, t547: f64, t807: f64, t9646: f64, t2236: f64, t66: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9711 = 0.12846167376791569079e-2_f64 * t1379 * t9709;
    let t9712 = t2689 * t3952;
    let t9714 = t1413 * t3889;
    let t9715 = t547 * t9714;
    let t9716 = t807 * t9715;
    let t9718 = t9646 * t547;
    let t9720 = 1.0_f64 / t66 / t2236;
    let t9721 = t9720 * t240;
    (t9711, t9712, t9716, t9718, t9720, t9721)
}
