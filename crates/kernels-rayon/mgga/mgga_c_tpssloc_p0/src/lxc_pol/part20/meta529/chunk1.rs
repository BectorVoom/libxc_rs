//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2064/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2064(t3795: f64, t40159: f64, t67: f64, t6924: f64, t246: f64, t12156: f64, t550: f64, t12012: f64, t12371: f64, t16398: f64, t12283: f64, t12426: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40160 = t40159 * t3795;
    let t40167 = t6924 * t67;
    let t40168 = t40167 * t246;
    let t40169 = t550 * t12156;
    let t40178 = t550 * t12012;
    let t40188 = t16398 * t12371;
    let t40190 = t12283 * t12426;
    (t40160, t40167, t40168, t40169, t40178, t40188, t40190)
}
