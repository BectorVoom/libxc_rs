//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2483/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2483(t1667: f64, t9709: f64, t14712: f64, t699: f64, t1113: f64, t136: f64, t50830: f64, t2403: f64, t4778: f64, t4723: f64, t9258: f64, t3297: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t50846 = t9709 * t1667;
    let t50848 = t699 * t14712;
    let t50851 = t136 * t1113 * t50830;
    let t50853 = t2403 * t4778;
    let t50854 = 0.27595e0_f64 * t50853;
    let t50857 = t4723 * t9258;
    let t50859 = t136 * t3297 * t50857;
    (t50846, t50848, t50851, t50853, t50854, t50857, t50859)
}
