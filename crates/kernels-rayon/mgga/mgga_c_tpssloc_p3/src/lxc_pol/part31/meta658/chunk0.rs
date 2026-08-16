//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1942/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1942(t16753: f64, t6605: f64, t815: f64, t16928: f64, t25084: f64, t16851: f64, t221: f64, t87420: f64, t16944: f64, t25154: f64, t841: f64, t87407: f64) -> (f64, f64, f64, f64, f64) {
    let t98801 = t6605 * t815 * t16753;
    let t98803 = t25084 * t16928;
    let t98808 = t87420 * t221 * t16851;
    let t98811 = t25154 * t221 * t16944;
    let t98814 = t87407 * t841 * t16851;
    (t98801, t98803, t98808, t98811, t98814)
}
