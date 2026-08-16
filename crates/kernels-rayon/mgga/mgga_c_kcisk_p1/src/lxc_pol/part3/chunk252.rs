//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 252/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk252(t1173: f64, t1175: f64, t1161: f64, t303: f64, t311: f64, t313: f64, t436: f64, t398: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1176 = t1173 * t1175;
    let t1178 = 0.29896666666666666667e0_f64 * t1161;
    let t1180 = f64::sqrt(t303);
    let t1181 = t1180 * t1175;
    let t1184 = t311 * t436 * t313;
    let t1185 = 0.82156666666666666667e-1_f64 * t1184;
    let t1186 = t79 * t398;
    (t1176, t1178, t1180, t1181, t1184, t1185, t1186)
}
