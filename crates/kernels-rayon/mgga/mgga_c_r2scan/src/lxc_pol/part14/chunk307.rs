//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 307/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk307(t322: f64, t1019: f64, t1020: f64, t1022: f64, t1024: f64, t1026: f64, t1028: f64, t1030: f64, t1035: f64, t343: f64, t352: f64, t855: f64, t372: f64, t381: f64, t404: f64, t408: f64, t412: f64, t459: f64, t466: f64, t470: f64, t880: f64, t881: f64, t900: f64, t902: f64, t913: f64, t955: f64, t970: f64, t989: f64) -> (f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t1039 = piecewise5(t323, t1019, t331, -0.64e0_f64 * t1020 - 0.8704e0_f64 * t1022 - 0.4607056813647e1_f64 * t1024 + 0.122462410087e2_f64 * t1026 - 0.957855118103e1_f64 * t1028 + 0.3101306810232e1_f64 * t1030 - 0.362942158544e0_f64 * t343 * t1020, -0.105e1_f64 * t855 * t1035 * t352);
    let t1044 = t880 - 0.2363e1_f64 * t881 * t970 + t372 * t955 - t381 - t404 + t408 + t412 - t900 - t459 - t902 + t466 + t470 - t913 - t989;
    (t1039, t1044)
}
