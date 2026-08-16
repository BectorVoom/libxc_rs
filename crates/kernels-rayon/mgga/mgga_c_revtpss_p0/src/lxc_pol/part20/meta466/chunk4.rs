//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1783/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1783(t10146: f64, t123: f64, t3915: f64, t676: f64, t10008: f64, t1358: f64, t212: f64, t689: f64, t1359: f64, t39501: f64, t10171: f64, t1424: f64, t4071: f64, t4076: f64, t4077: f64, t4131: f64, t4132: f64, t47521: f64, t47525: f64, t47527: f64, t47531: f64, t47534: f64, t47537: f64, t47540: f64, t47546: f64, t47550: f64, t9657: f64, t9659: f64) -> f64 {
    let t47554 = t3915 * t123 * t676 * t10146;
    let t47558 = t689 * t212 * t10008 * t1358;
    let t47561 = 0.56911289235245161963e-1_f64 * t39501 * t1359;
    let t47566 = -0.13878983423218070567e-1_f64 * t47521 + 0.78059524315062264152e-1_f64 * t47525 + 0.15611904863012452831e0_f64 * t47527 + 0.23417857294518679245e0_f64 * t47531 + 0.39029762157531132075e-2_f64 * t47534 + 0.13170898365871023197e0_f64 * t47537 + 0.65854491829355115985e-1_f64 * t47540 - 0.39512695097613069592e1_f64 * t10171 * t4132 - 0.15805078039045227836e2_f64 * t4071 * t9659 + 0.39512695097613069591e1_f64 * t1424 * t4076 * t47546 - 0.11708928647259339623e0_f64 * t47550 - 0.39029762157531132076e-1_f64 * t47554 - 0.21951497276451705328e-1_f64 * t47558 + t47561 - 0.23707617058567841754e2_f64 * t1424 * t9657 * t4077 * t4131;
    t47566
}
