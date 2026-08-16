//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1266/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1266(t23062: f64, t30700: f64, t240: f64, t241: f64, t2627: f64, t812: f64, t2617: f64, t30713: f64, t814: f64, t835: f64, t30716: f64, t22690: f64, t23122: f64, t6619: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t112784 = t23062 * t30700;
    let t112792 = t812 * t2627 * t240 * t241;
    let t112797 = t2617 * t30713;
    let t112802 = t812 * t814 * t835 * t241;
    let t112803 = t112802 * t30716;
    let t112804 = 7.0_f64 / 1152.0_f64 * t112803;
    let t112818 = t23122 * t22690 * t6619 * t776;
    (t112784, t112792, t112797, t112802, t112804, t112818)
}
