//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 436/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk436(t1156: f64, t140: f64, t1190: f64, t1215: f64, t453: f64, t673: f64, t1193: f64, t1182: f64, t209: f64, t463: f64, t205: f64, t1184: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4467 = t1156 * t140;
    let t4477 = t1190 * t1215;
    let t4504 = t673 * t453;
    let t4505 = t1193 * t4504;
    let t4510 = t1182 * t209;
    let t4516 = t463 * t463;
    let t4517 = 1.0_f64 / t4516;
    let t4518 = t205 * t4517;
    let t4522 = t1184 * t209;
    (t4467, t4477, t4505, t4510, t4517, t4518, t4522)
}
