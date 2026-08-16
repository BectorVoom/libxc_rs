//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1348/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1348(t55921: f64, t7245: f64, t12571: f64, t27331: f64, t2240: f64, t29473: f64, t33: f64, t111: f64, t29485: f64, t112: f64, t29865: f64, t1851: f64, t8119: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104953 = t55921 * t7245;
    let t104958 = t12571 * t27331;
    let t104968 = t2240 * t33 * t29473;
    let t104990 = t29485 * t111;
    let t105105 = t29865 * t112;
    let t105131 = t1851 * t8119;
    (t104953, t104958, t104968, t104990, t105105, t105131)
}
