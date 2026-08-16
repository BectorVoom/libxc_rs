//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1144/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1144(t30633: f64, t865: f64, t23270: f64, t1888: f64, t794: f64, t8331: f64, t6562: f64, t225: f64, t258: f64, t6624: f64, t214: f64, t1880: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30634 = t30633 * t865;
    let t30635 = t23270 * t30634;
    let t30637 = 0.3289868133696452873e-1_f64 * t1888 * t30635;
    let t30638 = t794 * t8331;
    let t30640 = 0.82246703342411321825e-2_f64 * t6562 * t30638;
    let t30642 = t6624 * t225 * t258;
    let t30643 = t214 * t30642;
    let t30645 = 0.16449340668482264365e-1_f64 * t1880 * t30643;
    (t30634, t30635, t30637, t30638, t30640, t30642, t30643, t30645)
}
