//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1033/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1033(t11156: f64, t11157: f64, t912: f64, t10982: f64, t10989: f64, t11049: f64, t10992: f64, t10994: f64, t11041: f64, t11044: f64, t11047: f64, t11051: f64, t8647: f64, t8796: f64, t8797: f64) -> (f64, f64, f64) {
    let t11158 = t11156 * t11157;
    let t11160 = 0.10254018858216406658e4_f64 * t912 * t11158;
    let t11169 = 0.20128333333333333334e0_f64 * t10982;
    let t11172 = 0.11038e0_f64 * t10989;
    let t11179 = 0.22076e0_f64 * t11049;
    let t11181 = t11172 - 0.82785e-1_f64 * t10992 - 0.91983333333333333334e-1_f64 * t10994 - 0.11038e0_f64 * t8647 - t8796 - t8797 + 0.258925e1_f64 * t11041 - 0.49671e0_f64 * t11044 + 0.16557e0_f64 * t11047 - t11179 + 0.36793333333333333334e-1_f64 * t11051;
    (t11160, t11169, t11181)
}
