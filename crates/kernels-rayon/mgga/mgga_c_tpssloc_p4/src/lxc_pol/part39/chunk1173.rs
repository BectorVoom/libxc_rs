//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1173/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1173(t2250: f64, t4723: f64, t3240: f64, t123: f64) -> (f64, f64) {
    let t14740 = t4723 * t2250;
    let t14741 = t3240 * t14740;
    let t14742 = t123 * t14741;
    (t14740, t14742)
}
