//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 469/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk469(t53: f64, t1794: f64, t3985: f64, t1797: f64, t912: f64, t3878: f64, t814: f64, t1395: f64, t280: f64, t57: f64, t815: f64, t1802: f64, t3998: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t54 = t53 <= zeta_threshold;
    let t5850 = t3985 * t1794;
    let t5855 = t912 * t1797;
    let t5860 = -2.0_f64 * t814 - 6.0_f64 * t3878;
    let t5864 = piecewise3(t54, 0.0_f64, -8.0_f64 / 27.0_f64 * t5850 * t280 + 16.0_f64 / 9.0_f64 * t1395 * t815 + 4.0_f64 / 9.0_f64 * t5855 * t280 + 4.0_f64 / 3.0_f64 * t57 * t5860);
    let t5865 = t3998 * t1802;
    (t5860, t5864, t5865)
}
