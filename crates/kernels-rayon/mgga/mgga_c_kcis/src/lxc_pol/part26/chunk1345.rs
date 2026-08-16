//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1345/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1345(t22351: f64, t27520: f64, t28594: f64, t28597: f64, t102985: f64, t102987: f64, t102989: f64, t102991: f64, t102993: f64, t102995: f64, t102997: f64, t102999: f64, t103002: f64) -> (f64, f64, f64) {
    let t103004 = t27520 * t22351;
    let t103006 = t28594 * t28597;
    let t103008 = 0.89930555555555555557e-2_f64 * t102985 - 0.4046875e-1_f64 * t102987 - 0.28777777777777777779e0_f64 * t102989 + 0.59953703703703703705e-2_f64 * t102991 + 0.33333333333333333333e0_f64 * t102993 - 0.41666666666666666667e-1_f64 * t102995 + 0.68347222222222222224e0_f64 * t102997 - 0.20833333333333333333e-1_f64 * t102999 - 0.809375e-1_f64 * t103002 + 0.375e0_f64 * t103004 + 0.5e0_f64 * t103006;
    (t103004, t103006, t103008)
}
