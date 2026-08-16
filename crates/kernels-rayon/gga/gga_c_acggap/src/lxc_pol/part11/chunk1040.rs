//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1040/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1040(t30325: f64, t1165: f64, t2068: f64, t20935: f64, t7351: f64, t30318: f64, t532: f64, t1569: f64, t7614: f64, t1988: f64, t8838: f64, t1089: f64, t1459: f64, t33878: f64, t598: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34288 = 0.18868855373762491241e-2_f64 * t30325;
    let t34291 = t2068 * t1165 * t7351 * t20935;
    let t34293 = t30318 * t532;
    let t34295 = t7614 * t1569;
    let t34296 = 0.16006300097412701803e-1_f64 * t34295;
    let t34297 = t1988 * t8838;
    let t34298 = 0.10718504529517434243e-2_f64 * t34297;
    let t34301 = t598 * t1089 * t1459 * t33878;
    (t34288, t34291, t34293, t34296, t34298, t34301)
}
