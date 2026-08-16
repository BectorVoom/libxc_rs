//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 910/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk910(t10495: f64, t10498: f64, t10501: f64, t10503: f64, t10507: f64, t10511: f64, t10513: f64, t10978: f64, t10984: f64, t10987: f64, t10989: f64, t10992: f64, t10998: f64, t11000: f64, t865: f64, t887: f64) -> f64 {
    let t11002 = 0.39512695097613069591e1_f64 * t865 * t10495 + 0.21951497276451705329e-1_f64 * t10498 + t10501 - t10503 - 0.34697458558045176417e-2_f64 * t10507 + 0.39029762157531132076e-1_f64 * t10511 - 0.19756347548806534796e1_f64 * t10513 * t887 - 0.65854491829355115987e0_f64 * t865 * t10978 + t10984 - t10987 + 0.16463622957338778996e-1_f64 * t10989 + 0.32927245914677557992e-1_f64 * t10992 + 0.58544643236296698113e-1_f64 * t10998 - 0.21951497276451705329e-1_f64 * t11000;
    t11002
}
