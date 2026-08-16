//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1396/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1396(t12252: f64, t12261: f64, t12263: f64, t12265: f64, t12459: f64, t12460: f64, t16731: f64, t16852: f64, t16855: f64, t16858: f64, t16860: f64, t16863: f64, t16865: f64, t16887: f64, t16890: f64, t16895: f64, t16898: f64, t16901: f64, t16904: f64, t17061: f64, t17066: f64, t17083: f64) -> f64 {
    let t17085 = 0.23154444444444444444e-1_f64 * t12252 + 0.23154444444444444444e0_f64 * t12261 - 0.69463333333333333333e-1_f64 * t12263 - 0.13892666666666666667e0_f64 * t12265 + 0.264729375e1_f64 * t16852 - 0.157790625e0_f64 * t16855 - 0.3529725e1_f64 * t16858 - 0.17648625e1_f64 * t16860 + 0.6311625e0_f64 * t16863 + 0.31558125e0_f64 * t16865 + t17061 - 0.34431666666666666667e0_f64 * t16731 + 0.20839e0_f64 * t16887 + 0.62517e0_f64 * t16890 - t17066 - 0.69463333333333333334e-1_f64 * t16895 - t12459 - t12460 - 0.34731666666666666667e-1_f64 * t16898 - 0.20839e0_f64 * t16901 + 0.41678e0_f64 * t16904 + t17083;
    t17085
}
