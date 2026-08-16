//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 905/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk905(t1652: f64, t2392: f64, t262: f64, t8620: f64, t8577: f64, t9165: f64, t1971: f64, t236: f64, t36489: f64, t495: f64, t6108: f64, t16503: f64, t22971: f64, t552: f64, t8425: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45166 = t2392 * t1652;
    let t45167 = t262 * t45166;
    let t45168 = t8620 * t45167;
    let t45170 = t8577 * t9165;
    let t45175 = t36489 * t1971 * t236 * t6108 * t495;
    let t45179 = t16503 * t22971 * t552 * t8425;
    (t45166, t45167, t45168, t45170, t45175, t45179)
}
