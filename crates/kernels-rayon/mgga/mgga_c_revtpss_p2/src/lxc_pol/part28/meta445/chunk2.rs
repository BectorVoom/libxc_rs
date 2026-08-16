//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1681/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1681(t12252: f64, t12261: f64, t12263: f64, t12265: f64, t12542: f64, t12543: f64, t16731: f64, t16852: f64, t16855: f64, t16858: f64, t16860: f64, t16863: f64, t16865: f64, t16887: f64, t16890: f64, t16895: f64, t16898: f64, t16901: f64, t16904: f64, t17126: f64, t17131: f64, t17148: f64) -> f64 {
    let t17150 = 0.18396666666666666667e-1_f64 * t12252 + 0.18396666666666666667e0_f64 * t12261 - 0.5519e-1_f64 * t12263 - 0.11038e0_f64 * t12265 + 0.19419375e1_f64 * t16852 - 0.412621875e-1_f64 * t16855 - 0.258925e1_f64 * t16858 - 0.1294625e1_f64 * t16860 + 0.16504875e0_f64 * t16863 + 0.82524375e-1_f64 * t16865 + t17126 - 0.20128333333333333333e0_f64 * t16731 + 0.16557e0_f64 * t16887 + 0.49671e0_f64 * t16890 - t17131 - 0.5519e-1_f64 * t16895 - t12542 - t12543 - 0.27595e-1_f64 * t16898 - 0.16557e0_f64 * t16901 + 0.33114e0_f64 * t16904 + t17148;
    t17150
}
