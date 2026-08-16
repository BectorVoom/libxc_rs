//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1350/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1350(t26090: f64, t7235: f64, t2320: f64, t569: f64, t7221: f64, t94369: f64, t94371: f64, t94374: f64, t94376: f64, t94940: f64, t94942: f64, t94944: f64, t94994: f64, t94998: f64, t95001: f64, t95005: f64, t95008: f64, t95011: f64, t95013: f64, t95015: f64, t95017: f64, t95020: f64, t95023: f64) -> f64 {
    let t95025 = 3.0_f64 * t7235 * t26090;
    let t95026 = -3.0_f64 * t2320 * t7221 + t569 * t94994 + t94369 - t94371 - t94374 + t94376 + t94940 - t94942 - t94944 - t94998 + t95001 + t95005 + t95008 - t95011 - t95013 - t95015 - t95017 + t95020 + t95023 + t95025;
    t95026
}
