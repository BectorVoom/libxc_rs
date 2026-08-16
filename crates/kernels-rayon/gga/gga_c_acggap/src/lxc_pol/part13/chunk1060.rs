//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1060/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1060(t30624: f64, t34522: f64, t34526: f64, t34529: f64, t34532: f64, t34535: f64, t34538: f64, t34539: f64, t34541: f64, t34543: f64, t34545: f64, t34547: f64, t34549: f64, t34553: f64, t34557: f64, t34559: f64, t34562: f64, t34563: f64) -> f64 {
    let t34565 = 0.18868855373762491241e-2_f64 * t34522 + 0.41930789719472202758e-3_f64 * t34526 + t34529 / 48.0_f64 + t34532 / 48.0_f64 - t34535 + 0.42874018118069736972e-3_f64 * t30624 + t34538 - 0.17149607247227894789e-2_f64 * t34539 + 0.25724410870841842183e-2_f64 * t34541 - 0.17149607247227894789e-1_f64 * t34543 + 0.51448821741683684367e-2_f64 * t34545 - 0.17149607247227894789e-2_f64 * t34547 - 0.80031500487063509014e-2_f64 * t34549 + 0.94344276868812456204e-3_f64 * t34553 + t34557 + 0.31448092289604152068e-2_f64 * t34559 + t34562 + 0.13719685797782315831e-1_f64 * t34563;
    t34565
}
