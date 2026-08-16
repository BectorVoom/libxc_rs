//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1391/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1391(t12252: f64, t12261: f64, t12263: f64, t12265: f64, t12349: f64, t12352: f64, t16731: f64, t16852: f64, t16855: f64, t16858: f64, t16860: f64, t16863: f64, t16865: f64, t16883: f64, t16887: f64, t16890: f64, t16893: f64, t16895: f64, t16898: f64, t16901: f64, t16904: f64, t16940: f64) -> f64 {
    let t16942 = 0.18257037037037037037e-1_f64 * t12252 + 0.18257037037037037037e0_f64 * t12261 - 0.54771111111111111111e-1_f64 * t12263 - 0.10954222222222222222e0_f64 * t12265 + 0.142419375e1_f64 * t16852 - 0.76790625e-1_f64 * t16855 - 0.1898925e1_f64 * t16858 - 0.9494625e0_f64 * t16860 + 0.3071625e0_f64 * t16863 + 0.15358125e0_f64 * t16865 + t16883 - 0.19931111111111111111e0_f64 * t16731 + 0.16431333333333333333e0_f64 * t16887 + 0.49293999999999999999e0_f64 * t16890 - t16893 - 0.54771111111111111112e-1_f64 * t16895 - t12349 - t12352 - 0.27385555555555555556e-1_f64 * t16898 - 0.16431333333333333333e0_f64 * t16901 + 0.32862666666666666666e0_f64 * t16904 + t16940;
    t16942
}
