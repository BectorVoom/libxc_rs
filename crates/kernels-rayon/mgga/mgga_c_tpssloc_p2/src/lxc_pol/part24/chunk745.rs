//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 745/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk745(t1922: f64, t968: f64, t1920: f64, t221: f64, t60: f64, t1926: f64) -> (f64, f64, f64, f64) {
    let t6683 = t968 * t1922;
    let t6685 = 0.27415567780803773942e-2_f64 * t1920 * t6683;
    let t6686 = t221 * t60;
    let t6687 = t1926 * t6686;
    (t6683, t6685, t6686, t6687)
}
