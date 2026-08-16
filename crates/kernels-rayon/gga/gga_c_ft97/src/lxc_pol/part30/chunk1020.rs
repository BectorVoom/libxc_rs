//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1020/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1020(t17836: f64, t33423: f64, t141051: f64, t6832: f64, t27658: f64, t123650: f64, t140884: f64, t140885: f64, t140929: f64, t140941: f64, t141112: f64, t141123: f64, t141171: f64, t141172: f64, t141176: f64, t150372: f64, t17864: f64, t2387: f64, t27529: f64, t27539: f64, t27557: f64, t27561: f64, t27682: f64, t36792: f64, t37481: f64, t7205: f64, t92354: f64, t98545: f64, sigma2: f64) -> (f64, f64) {
    let t150429 = t17836 * t33423;
    let t150436 = t141051 * t6832;
    let t150437 = t27658 * t150436;
    let t150460 = -0.13649345781532662578e-3_f64 * t150429 * t140885 * t27529 + 0.20474018672298993869e-3_f64 * t140884 * t140885 * t17864 + 0.37842536879785276493e-4_f64 * t150437 - 0.13200366700519885118e-5_f64 * t141171 * t141172 * t123650 + 0.29693535778629056444e-3_f64 * t141176 * t98545 * t123650 - 0.17816121467177433867e-2_f64 * t141112 * t27682 + 0.79202200203119310706e-6_f64 * t141171 * t36792 * t27557 + 0.6595632919850939344e-7_f64 * t2387 * t92354 * t37481 * sigma2 * t36792 * t27561 - 0.45497819271775541929e-4_f64 * t141123 * t7205 * t150372 * t27539 - t140929 + t140941;
    (t150436, t150460)
}
