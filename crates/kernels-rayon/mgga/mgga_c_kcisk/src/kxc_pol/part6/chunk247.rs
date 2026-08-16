//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 247/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk247(t1161: f64, t303: f64, t311: f64, t313: f64, t436: f64, t398: f64, t79: f64) -> (f64, f64, f64, f64, f64) {
    let t1178 = 0.29896666666666666667e0_f64 * t1161;
    let t1180 = f64::sqrt(t303);
    let t1184 = t311 * t436 * t313;
    let t1185 = 0.82156666666666666667e-1_f64 * t1184;
    let t1186 = t79 * t398;
    (t1178, t1180, t1184, t1185, t1186)
}
