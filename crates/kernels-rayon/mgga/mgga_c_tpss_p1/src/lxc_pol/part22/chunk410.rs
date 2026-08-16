//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 410/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk410(t1369: f64, t1381: f64, t1385: f64, t759: f64, t761: f64, t771: f64, t794: f64, t797: f64) -> f64 {
    let t1388 = -t759 - t761 * t1369 / 48.0_f64 - t771 * t1381 / 3072.0_f64 - t794 - t797 * t1385 / 768.0_f64;
    t1388
}
