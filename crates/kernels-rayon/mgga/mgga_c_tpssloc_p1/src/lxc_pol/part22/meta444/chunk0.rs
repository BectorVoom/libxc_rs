//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1794/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1794(t16132: f64, t1825: f64, t1352: f64, t19743: f64, t19660: f64, t118: f64, t6330: f64, t794: f64, t12202: f64, t19631: f64, t210: f64, t214: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19756 = t16132 * t1825;
    let t19761 = t19743 * t1352;
    let t19763 = t19660 * t1352;
    let t19767 = t118 * t794 * t6330;
    let t19768 = t12202 * t19767;
    let t19771 = t210 * t214 * t19631;
    (t19756, t19761, t19763, t19767, t19768, t19771)
}
