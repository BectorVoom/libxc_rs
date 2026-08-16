//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2098/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2098(t91402: f64, t22804: f64, t26277: f64, t225: f64, t26221: f64, t22674: f64, t22892: f64, t26189: f64, t26329: f64, t26229: f64, t22724: f64, t26344: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t91403 = 7.0_f64 / 72.0_f64 * t91402;
    let t91404 = t22804 * t26277;
    let t91441 = t26221 * t225;
    let t91486 = t22892 * t22674 * t26189;
    let t91487 = 0.16449340668482264365e-1_f64 * t91486;
    let t91488 = t26329 * t225;
    let t91491 = t26229 * t225;
    let t91531 = t22724 * t26344;
    (t91403, t91404, t91441, t91487, t91488, t91491, t91531)
}
