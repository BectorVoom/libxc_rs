//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1109/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1109(t26960: f64, t26974: f64, t27077: f64, t27849: f64, t28161: f64, t28190: f64, t28204: f64, t28920: f64, t29094: f64, t29123: f64, t29127: f64, t7772: f64, t7788: f64, t8087: f64, t8095: f64) -> f64 {
    let t29143 = 0.23168402777777777778e-3_f64 * t26960 * t29123 + 0.34752604166666666667e-3_f64 * t7788 * t29127 - 0.92835860883789062501e-5_f64 * t27077 * t29094 - 0.13913205078125e-3_f64 * t7772 * t29094 + t26974 + 0.17411041666666666666e-2_f64 * t28920 + 0.23168402777777777778e-3_f64 * t28161 + 0.15476481481481481481e-2_f64 * t27849 + 0.69505208333333333334e-3_f64 * t28190 * t8095 + 0.69505208333333333334e-3_f64 * t28190 * t8087 + 0.92754700520833333334e-4_f64 * t28204 * t8087;
    t29143
}
