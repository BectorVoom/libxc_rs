//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3107/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3107(t43777: f64, t43855: f64, t43859: f64, t43861: f64, t43863: f64, t50903: f64, t50905: f64, t50907: f64, t50919: f64, t50921: f64, t50948: f64, t50950: f64, t50952: f64, t50954: f64) -> f64 {
    let t64374 = t43777 - 0.30661111111111111111e-1_f64 * t43855 - 0.49057777777777777778e0_f64 * t43859 + 0.91983333333333333333e-1_f64 * t43861 + 0.18396666666666666667e0_f64 * t43863 - 0.80513333333333333336e0_f64 * t50903 - 0.40256666666666666668e0_f64 * t50905 - 0.12077e1_f64 * t50907 - 0.35783703703703703705e0_f64 * t50919 - 0.22364814814814814815e0_f64 * t50921 + 0.10735111111111111112e1_f64 * t50948 + 0.26837777777777777778e0_f64 * t50950 + 0.13418888888888888889e0_f64 * t50952 + 0.80513333333333333335e0_f64 * t50954;
    t64374
}
