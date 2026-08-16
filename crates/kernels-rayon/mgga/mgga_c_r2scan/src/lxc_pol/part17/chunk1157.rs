//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1157/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1157(t3308: f64, t574: f64, t8779: f64, t12476: f64, t37685: f64, t10776: f64, t9165: f64, t9169: f64, t12479: f64, t37641: f64, t10772: f64, t9261: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43032 = t574 * t3308 * t8779;
    let t43034 = t37685 * t12476;
    let t43037 = t10776 * t3308 * t9165;
    let t43040 = t10776 * t3308 * t9169;
    let t43042 = t37641 * t12479;
    let t43045 = t10772 * t3308 * t9261;
    (t43032, t43034, t43037, t43040, t43042, t43045)
}
