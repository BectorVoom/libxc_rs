//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1245/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1245(t38150: f64, t38170: f64, t38177: f64, t38657: f64, t38661: f64, t40238: f64, t40248: f64, t40251: f64, t41762: f64, t41763: f64, t43677: f64, t43682: f64) -> f64 {
    let t44500 = 0.43663693315433241794e-2_f64 * t43677 + 0.16262400898971305031e-3_f64 * t38150 - t40238 - 0.86682217400542685632e-1_f64 * t43682 - t41762 - t41763 + t38657 + t40248 + 0.45022119329691164871e0_f64 * t38170 + t38661 - 0.65854491829355115986e-1_f64 * t38177 - 0.7141495379651092646e0_f64 * t40251;
    t44500
}
