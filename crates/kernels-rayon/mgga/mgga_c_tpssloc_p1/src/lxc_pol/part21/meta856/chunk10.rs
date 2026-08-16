//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3106/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3106(t50846: f64, t50848: f64, t50853: f64, t63918: f64, t63921: f64, t63924: f64, t63927: f64, t63930: f64, t63933: f64, t63936: f64, t63939: f64, t63997: f64, t64003: f64, t64006: f64, t64009: f64) -> f64 {
    let t64358 = -0.8585111111111111111e-1_f64 * t63918 - 0.5519e-1_f64 * t63921 - 0.27595e-1_f64 * t63924 - 0.16557e0_f64 * t63927 + 0.36793333333333333333e-1_f64 * t63930 + 0.44152e0_f64 * t63933 + 0.49671e0_f64 * t63936 + 0.198684e1_f64 * t63939 + 0.258925e1_f64 * t63997 - 0.49057777777777777779e0_f64 * t50846 - 0.11038e0_f64 * t50848 + 0.36793333333333333334e0_f64 * t50853 - 0.66228e0_f64 * t64003 + 0.198684e1_f64 * t64006 + 0.16504875e0_f64 * t64009;
    t64358
}
