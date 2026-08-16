//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2485/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2485(t14730: f64, t9288: f64, t1113: f64, t136: f64, t12606: f64, t3242: f64, t607: f64, t3297: f64, t123: f64, t3240: f64, t50857: f64) -> (f64, f64, f64, f64, f64) {
    let t50879 = t14730 * t9288;
    let t50881 = t136 * t1113 * t50879;
    let t50884 = t3242 * t12606 * t607;
    let t50886 = t136 * t3297 * t50884;
    let t50897 = t123 * t3240 * t50857;
    (t50879, t50881, t50884, t50886, t50897)
}
