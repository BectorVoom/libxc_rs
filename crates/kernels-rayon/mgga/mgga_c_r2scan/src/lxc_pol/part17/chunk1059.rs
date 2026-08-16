//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1059/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1059(t3437: f64, t58: f64, t10929: f64, t37434: f64, t424: f64, t3428: f64, t3430: f64, t6826: f64, t761: f64, t10659: f64, t10943: f64, t6818: f64) -> (f64, f64, f64, f64, f64) {
    let t37435 = t3437 * t58;
    let t37438 = t37434 * t37435 * t424 * t10929;
    let t37442 = t6826 * t761 * t3428 * t3430;
    let t37444 = t10943 * t10659;
    let t37447 = t6818 * t3428 * t3430;
    (t37435, t37438, t37442, t37444, t37447)
}
