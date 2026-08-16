//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1311/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1311(t6552: f64, t6637: f64, t776: f64, t81658: f64, t1888: f64, t232: f64, t40955: f64, t6646: f64, t23110: f64, t23176: f64, t23185: f64, t252: f64, t9660: f64) -> (f64, f64, f64, f64) {
    let t81661 = t6552 * t6637 * t81658 * t776;
    let t81667 = t1888 * t6646 * t40955 * t232;
    let t81670 = t23185 * t23110 * t23176;
    let t81672 = t252 * t9660;
    (t81661, t81667, t81670, t81672)
}
