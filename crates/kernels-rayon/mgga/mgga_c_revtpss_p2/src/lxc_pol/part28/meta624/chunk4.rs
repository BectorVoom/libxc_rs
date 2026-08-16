//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2218/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2218(t4797: f64, t7131: f64, t1068: f64, t15970: f64, t27493: f64, t4788: f64, t93597: f64, t93687: f64, t93689: f64, t93694: f64, t93696: f64, t93702: f64, t93704: f64, t93713: f64, t93718: f64, t93720: f64) -> f64 {
    let t100230 = t4797 * t7131;
    let t100233 = -0.3811023832717309953e-3_f64 * t93687 + 0.57165357490759649296e-3_f64 * t93689 + 0.57165357490759649296e-3_f64 * t27493 * t15970 - 0.30488190661738479624e-2_f64 * t93597 * t4788 - t93694 / 162.0_f64 - t93696 / 648.0_f64 + t93702 / 864.0_f64 + t93704 / 648.0_f64 + 0.57165357490759649296e-3_f64 * t93713 + 0.30488190661738479624e-2_f64 * t93718 + 0.19055119163586549765e-3_f64 * t93720 + 0.57165357490759649296e-3_f64 * t100230 * t1068;
    t100233
}
