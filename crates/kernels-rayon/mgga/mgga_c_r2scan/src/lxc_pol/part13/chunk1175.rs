//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1175/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1175(t39885: f64, t8243: f64, t2605: f64, t37699: f64, t10833: f64, t980: f64, t38069: f64, t38074: f64, t38076: f64, t38079: f64, t40090: f64, t40092: f64, t40095: f64, t40098: f64, t40100: f64) -> f64 {
    let t40102 = t39885 * t8243;
    let t40103 = 0.19514881078765566037e-1_f64 * t40102;
    let t40107 = t37699 * t2605;
    let t40109 = t980 * t10833;
    let t40111 = 0.55889527443754549494e0_f64 * t40090 + 0.10401866088065122276e1_f64 * t40092 + 0.13002332610081402845e0_f64 * t40095 + 0.43663693315433241792e-2_f64 * t40098 - 0.13099107994629972538e-1_f64 * t40100 + t40103 - t38069 + 0.34672886960217074253e0_f64 * t38074 + 0.69345773920434148506e0_f64 * t38076 + 0.11557628986739024751e0_f64 * t38079 + 0.29272321618148349056e-1_f64 * t40107 + 0.42377972951376424087e0_f64 * t40109;
    t40111
}
