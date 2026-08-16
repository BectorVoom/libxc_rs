//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1985/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1985(t101593: f64, t101618: f64, t101634: f64, t101656: f64, t101672: f64, t101687: f64, t101705: f64, t101734: f64, t101751: f64, t2054: f64, t26690: f64, t26700: f64, t26703: f64, t4147: f64, t4268: f64, t4273: f64, t59519: f64, t85129: f64, t855: f64, t858: f64, t866: f64, t98941: f64, t98945: f64, t98963: f64, t98966: f64) -> f64 {
    let t101761 = -t85129 - 0.15352717957250113407e0_f64 * t98941 - 0.16449340668482264365e-1_f64 * t98945 + 4.0_f64 * t4268 * t26703 + 4.0_f64 * t4147 * t26690 - 0.19739208802178717238e0_f64 * t98963 - t101593 * t866 - 0.16449340668482264365e-1_f64 * t98966 - t855 * t858 * (t101618 + t101634 + t101656 + t101672 + t101687 + t101705 + t101734 + t101751) + 4.0_f64 * t26700 * t4273 - 2.0_f64 * t59519 * t2054;
    t101761
}
