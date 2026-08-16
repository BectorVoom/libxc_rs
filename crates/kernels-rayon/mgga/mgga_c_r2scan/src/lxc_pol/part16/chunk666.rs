//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 666/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk666(t4733: f64, t4736: f64, t4739: f64, t4849: f64, t4850: f64, t4851: f64, t4852: f64, t4853: f64, t453: f64, t1379: f64, t445: f64, t76: f64) -> (f64, f64, f64, f64) {
    let t4854 = -0.34523333333333333333e1_f64 * t4733 + 0.23015555555555555556e1_f64 * t4736 - 0.26851481481481481482e1_f64 * t4739 - t4849 + t4850 - t4851 - t4852 - t4853;
    let t4855 = t4854 * t453;
    let t4859 = 1.0_f64 / t1379 / t445;
    let t4860 = t76 * t4859;
    (t4854, t4855, t4859, t4860)
}
