//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 822/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk822(t5015: f64, t7031: f64, t7032: f64, t7051: f64, t4827: f64, t4839: f64, t4842: f64, t4845: f64, t4996: f64, t5000: f64, t5004: f64, t5008: f64, t5020: f64, t7025: f64, t7036: f64, t7095: f64) -> (f64, f64, f64, f64, f64) {
    let t8641 = 0.24415263074675393405e-3_f64 * t5015;
    let t8642 = 2.0_f64 * t7031;
    let t8643 = 0.48830526149350786811e-3_f64 * t7032;
    let t8644 = 16.0_f64 * t7051;
    let t8645 = t4996 - t5000 - t5004 - t5008 - t4827 + t4839 + t8641 - t5020 + t4842 - t7025 + t8642 + t8643 - t7036 - t4845 - t8644 - t7095;
    (t8641, t8642, t8643, t8644, t8645)
}
