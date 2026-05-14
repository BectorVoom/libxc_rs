//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 895/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk895<F: Float>(t17836: F, t33423: F, t141051: F, t6832: F, t27658: F, t123650: F, t140884: F, t140885: F, t140929: F, t140941: F, t141112: F, t141123: F, t141171: F, t141172: F, t141176: F, t150372: F, t17864: F, t2387: F, t27529: F, t27539: F, t27557: F, t27561: F, t27682: F, t36792: F, t37481: F, t7205: F, t92354: F, t98545: F, sigma2: F) -> (F, F) {
    let t150429 = t17836 * t33423;
    let t150436 = t141051 * t6832;
    let t150437 = t27658 * t150436;
    let t150460 = -0.13649345781532662578e-3 * t150429 * t140885 * t27529 + 0.20474018672298993869e-3 * t140884 * t140885 * t17864 + 0.37842536879785276493e-4 * t150437 - 0.13200366700519885118e-5 * t141171 * t141172 * t123650 + 0.29693535778629056444e-3 * t141176 * t98545 * t123650 - 0.17816121467177433867e-2 * t141112 * t27682 + 0.79202200203119310706e-6 * t141171 * t36792 * t27557 + 0.6595632919850939344e-7 * t2387 * t92354 * t37481 * sigma2 * t36792 * t27561 - 0.45497819271775541929e-4 * t141123 * t7205 * t150372 * t27539 - t140929 + t140941;
    (t150436, t150460)
}
