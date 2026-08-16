//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1928/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1928(t1888: f64, t232: f64, t58166: f64, t6646: f64, t16815: f64, t22986: f64, t2647: f64, t58226: f64, t23110: f64, t23185: f64, t28418: f64, t59331: f64) -> (f64, f64, f64, f64, f64) {
    let t98530 = t1888 * t6646 * t58166 * t232;
    let t98534 = t22986 * t6646 * t16815 * t2647;
    let t98546 = t1888 * t6646 * t58226 * t232;
    let t98549 = t23185 * t23110 * t28418;
    let t98553 = t1888 * t6646 * t59331 * t232;
    (t98530, t98534, t98546, t98549, t98553)
}
