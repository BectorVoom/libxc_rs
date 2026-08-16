//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1074/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1074(t27636: f64, t4457: f64, t6176: f64, t3805: f64, t7979: f64, t1600: f64, t27389: f64, t27392: f64, t27400: f64, t27420: f64, t27425: f64, t27429: f64, t27598: f64, t27626: f64, t7968: f64, t7978: f64) -> (f64, f64, f64, f64, f64) {
    let t27637 = t27636 * t4457;
    let t27638 = t6176 * t27637;
    let t27641 = t7979 * t3805;
    let t27642 = t1600 * t27641;
    let t27645 = -0.23214722222222222222e-2_f64 * t27389 - 0.13913205078125e-3_f64 * t7968 * t27598 - 0.15445601851851851852e-3_f64 * t7978 * t27626 - 0.17411041666666666666e-2_f64 * t27392 + 0.11607361111111111111e-2_f64 * t27400 - 0.38691203703703703703e-3_f64 * t27420 - 0.23214722222222222222e-2_f64 * t27425 - 0.23214722222222222222e-2_f64 * t27429 - 0.69505208333333333334e-3_f64 * t7978 * t27598 - 0.69505208333333333334e-3_f64 * t7978 * t27638 - 0.11584201388888888889e-3_f64 * t7978 * t27642;
    (t27637, t27638, t27641, t27642, t27645)
}
