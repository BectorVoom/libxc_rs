//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1103/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1103(t2842: f64, t37699: f64, t10698: f64, t2593: f64, t37759: f64, t38152: f64, t7418: f64, t38149: f64, t39469: f64, t37841: f64, t2833: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39642 = t37699 * t2842;
    let t39672 = t10698 * t2593;
    let t39679 = 0.11902492299418487743e0_f64 * t37759;
    let t39721 = t38152 * t7418;
    let t39723 = t38149 * t39469;
    let t39738 = 0.13506635798907349461e1_f64 * t37841;
    let t39739 = t545 * t2833;
    (t39642, t39672, t39679, t39721, t39723, t39738, t39739)
}
