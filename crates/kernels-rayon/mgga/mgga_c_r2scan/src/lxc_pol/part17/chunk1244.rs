//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1244/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1244(t38649: f64, t40228: f64, t41751: f64, t41756: f64, t41757: f64, t43654: f64, t43657: f64, t43660: f64, t43664: f64, t43667: f64, t43670: f64, t43672: f64) -> f64 {
    let t44492 = -0.13170898365871023197e1_f64 * t43654 + 0.65854491829355115984e0_f64 * t43657 - t41751 - 0.46574606203128791246e-1_f64 * t43660 + 0.65049603595885220124e-3_f64 * t40228 + t41756 + t41757 + 0.87327386630866483588e-2_f64 * t43664 - 0.43663693315433241794e-2_f64 * t43667 - 0.13099107994629972538e-1_f64 * t43670 - t38649 - 0.17336443480108537126e0_f64 * t43672;
    t44492
}
