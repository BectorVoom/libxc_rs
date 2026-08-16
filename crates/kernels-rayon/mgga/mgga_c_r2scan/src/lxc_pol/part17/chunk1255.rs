//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1255/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1255(t38792: f64, t38808: f64, t40808: f64, t40839: f64, t40846: f64, t41877: f64, t41885: f64, t41887: f64, t42524: f64, t42526: f64, t42528: f64, t42530: f64, t42532: f64, t42534: f64, t42536: f64, t42539: f64, t42541: f64, t42543: f64) -> f64 {
    let t44641 = -3.0_f64 / 2.0_f64 * t42524 + t42526 / 2.0_f64 + t42528 / 4.0_f64 + 44.0_f64 / 9.0_f64 * t40808 + t41877 + 3.0_f64 / 2.0_f64 * t42530 - t42532 - t42534 / 2.0_f64 - t42536 / 4.0_f64 + t40839 + 4.0_f64 / 3.0_f64 * t42539 + t38792 - t41885 + t41887 - 2.0_f64 / 3.0_f64 * t42541 + 2.0_f64 / 3.0_f64 * t42543 - t40846 + t38808;
    t44641
}
