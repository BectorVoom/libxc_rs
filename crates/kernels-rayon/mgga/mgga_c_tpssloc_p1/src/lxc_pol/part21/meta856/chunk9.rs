//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3105/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3105(t43748: f64, t63332: f64, t63334: f64, t63336: f64, t63886: f64, t63888: f64, t63891: f64, t63893: f64, t63896: f64, t63899: f64, t63903: f64, t63906: f64, t63909: f64, t63911: f64, t63914: f64) -> f64 {
    let t64342 = -0.89459259259259259257e-1_f64 * t63332 + 0.13418888888888888889e0_f64 * t63334 - 0.20128333333333333334e0_f64 * t63336 - 0.11038e0_f64 * t63886 - 0.30661111111111111112e-1_f64 * t63888 - 0.5519e-1_f64 * t63891 + 0.18396666666666666667e0_f64 * t63893 + 0.33114e0_f64 * t63896 + 0.14717333333333333333e0_f64 * t63899 - 0.8945925925925925926e-1_f64 * t43748 + 0.33114e0_f64 * t63903 + 0.16557e0_f64 * t63906 + 0.49671e0_f64 * t63909 + 0.91983333333333333334e-1_f64 * t63911 + 0.36793333333333333333e-1_f64 * t63914;
    t64342
}
