//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 307/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk307<F: Float>(t322: F, t1019: F, t1020: F, t1022: F, t1024: F, t1026: F, t1028: F, t1030: F, t1035: F, t343: F, t352: F, t855: F, t372: F, t381: F, t404: F, t408: F, t412: F, t459: F, t466: F, t470: F, t880: F, t881: F, t900: F, t902: F, t913: F, t955: F, t970: F, t989: F) -> (F, F) {
    let t323 = t322 <= F::new(0.0);
    let t331 = t322 <= F::new(0.25e1);
    let t1039 = piecewise5::<f64>(t323, t1019, t331, -F::new(0.64e0) * t1020 - F::new(0.8704e0) * t1022 - F::new(0.4607056813647e1) * t1024 + F::new(0.122462410087e2) * t1026 - F::new(0.957855118103e1) * t1028 + F::new(0.3101306810232e1) * t1030 - F::new(0.362942158544e0) * t343 * t1020, -F::new(0.105e1) * t855 * t1035 * t352);
    let t1044 = t880 - F::new(0.2363e1) * t881 * t970 + t372 * t955 - t381 - t404 + t408 + t412 - t900 - t459 - t902 + t466 + t470 - t913 - t989;
    (t1039, t1044)
}
