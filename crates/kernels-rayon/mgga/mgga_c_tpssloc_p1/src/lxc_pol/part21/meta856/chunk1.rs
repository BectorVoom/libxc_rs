//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3097/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3097(t50846: f64, t50848: f64, t50853: f64, t63918: f64, t63921: f64, t63924: f64, t63927: f64, t63930: f64, t63933: f64, t63936: f64, t63939: f64, t63997: f64, t64003: f64, t64006: f64, t64009: f64) -> f64 {
    let t64181 = -0.10805407407407407407e0_f64 * t63918 - 0.69463333333333333334e-1_f64 * t63921 - 0.34731666666666666667e-1_f64 * t63924 - 0.20839e0_f64 * t63927 + 0.46308888888888888889e-1_f64 * t63930 + 0.55570666666666666666e0_f64 * t63933 + 0.62517e0_f64 * t63936 + 0.250068e1_f64 * t63939 + 0.3529725e1_f64 * t63997 - 0.61745185185185185187e0_f64 * t50846 - 0.13892666666666666667e0_f64 * t50848 + 0.4630888888888888889e0_f64 * t50853 - 0.83356000000000000001e0_f64 * t64003 + 0.250068e1_f64 * t64006 + 0.6311625e0_f64 * t64009;
    t64181
}
