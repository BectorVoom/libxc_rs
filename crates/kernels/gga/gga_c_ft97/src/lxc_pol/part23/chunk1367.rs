//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1367/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1367<F: Float>(t112223: F, t127281: F, t811: F, t108448: F, t31414: F, t684: F, t25057: F, t28628: F, t4125: F, t31526: F, t6051: F, t1109: F, t1196: F, t820: F, t112133: F, t112137: F, t112138: F, t112159: F, t127283: F, t127301: F, t127365: F, t14721: F, t14729: F, t14766: F, t231: F, t25049: F, t25112: F, t27506: F, t28667: F, t31419: F, t6045: F, t70497: F, t70779: F, t83145: F, t98544: F, t98545: F) -> (F, F, F, F, F) {
    let t127389 = t112223 * t127281 * t811;
    let t127395 = t108448 * t31414 * t684;
    let t127410 = t25057 * t28628 * t4125;
    let t127417 = t31526 * t6051;
    let t127421 = t25057 * t1109 * t1196 * t820;
    let t127424 = -0.14817333576131687244e-1 * t112133 + t112137 + 0.29634667152263374487e-1 * t112138 - 0.4379116147943596799e1 * t70779 * t127389 + 0.4379116147943596799e1 * t70497 * t127283 + 0.1611184118048991131e0 * t112159 * t127395 + 0.13335600218518518519e0 * t98544 * t98545 * t31419 * t684 - 0.60010200983333333334e0 * t25112 * t6045 * t231 * t83145 - 0.10668480174814814815e1 * t25049 * t27506 * t28667 - 0.90613700826057446696e0 * t14729 * t127410 - 0.48327307107230638236e1 * t14766 * t127365 + 0.48327307107230638236e1 * t14729 * t127301 - 0.33339000546296296297e-1 * t127417 + 0.48327307107230638237e1 * t14721 * t127421;
    (t127389, t127395, t127410, t127421, t127424)
}
