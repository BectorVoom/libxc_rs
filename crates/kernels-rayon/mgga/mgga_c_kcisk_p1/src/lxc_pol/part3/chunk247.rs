//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 247/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk247(t222: f64, t1056: f64, t295: f64, t559: f64, t298: f64, t301: f64, t430: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t223 = t222 <= zeta_threshold;
    let t1155 = piecewise3(t223, 0.0_f64, t1056);
    let t1156 = t295 * t1155;
    let t1157 = t1156 * t559;
    let t1161 = t298 * t430 * t301;
    (t1156, t1157, t1161)
}
