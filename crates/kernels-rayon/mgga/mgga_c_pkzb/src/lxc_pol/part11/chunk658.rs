//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 658/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk658(t218: f64, t219: f64, t3757: f64, t334: f64, t3730: f64, t2212: f64, t2222: f64, t3017: f64, t3059: f64, t3732: f64, t3744: f64, t3748: f64, t3752: f64, t3754: f64) -> (f64, f64, f64, f64) {
    let t3759 = t218 * t219 * t3757;
    let t3761 = t334 * t3730;
    let t3763 = t218 * t219 * t3761;
    let t3765 = -0.9494625e0_f64 * t3744 + 0.1898925e1_f64 * t3748 + t2212 - 0.59793333333333333334e0_f64 * t3017 + 0.8969e0_f64 * t3732 + 0.15358125e0_f64 * t3752 + 0.3071625e0_f64 * t3754 + t2222 - 0.32862666666666666666e0_f64 * t3059 + 0.24647e0_f64 * t3759 + 0.24647e0_f64 * t3763;
    (t3759, t3761, t3763, t3765)
}
