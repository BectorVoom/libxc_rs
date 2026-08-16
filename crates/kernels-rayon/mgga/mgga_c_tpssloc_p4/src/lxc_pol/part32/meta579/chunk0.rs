//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1958/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1958(t2140: f64, t6169: f64, t1748: f64, t27611: f64, t27617: f64, t27622: f64, t27629: f64, t27684: f64, t27711: f64, t29585: f64, t29594: f64, t29597: f64, t29601: f64, t467: f64, t488: f64, t7326: f64, t8040: f64) -> (f64, f64) {
    let t29606 = t6169 * t2140;
    let t29610 = -t27617 * t1748 / 1152.0_f64 + t27611 / 1152.0_f64 + 11.0_f64 / 108.0_f64 * t29585 * t467 - 0.16149102437656156342e-2_f64 * t27711 * t8040 - 0.20186378047070195428e-3_f64 * t27684 * t8040 + 0.10093189023535097714e-3_f64 * t7326 * t29594 - t29597 * t488 / 144.0_f64 + 19.0_f64 / 864.0_f64 * t29601 * t488 - 0.20186378047070195428e-3_f64 * t27629 * t8040 + t29606 * t488 / 1536.0_f64 - t27622 / 1728.0_f64;
    (t29606, t29610)
}
