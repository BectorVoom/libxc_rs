//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1286/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1286(t112983: f64, t1888: f64, t25262: f64, t6646: f64, t112991: f64, t112997: f64, t32827: f64, t6547: f64, t1880: f64, t1894: f64, t214: f64, t25160: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t118730 = 0.82246703342411321825e-2_f64 * t112983;
    let t118735 = 0.16449340668482264365e-1_f64 * t1888 * t6646 * t25262;
    let t118736 = 0.38381794893125283518e-1_f64 * t112991;
    let t118737 = 0.82246703342411321825e-2_f64 * t112997;
    let t118738 = t6547 * t32827;
    let t118739 = 0.38381794893125283518e-1_f64 * t118738;
    let t118743 = 0.16449340668482264365e-1_f64 * t1880 * t214 * t1894 * t25160;
    (t118730, t118735, t118736, t118737, t118739, t118743)
}
