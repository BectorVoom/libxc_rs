//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1004/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1004(t10777: f64, t10779: f64, t10782: f64, t10786: f64, t10790: f64, t10795: f64, t10799: f64, t10803: f64, t10805: f64, t10809: f64, t10813: f64, t2173: f64, t3626: f64, t8289: f64, t8293: f64, t8314: f64) -> f64 {
    let t10816 = 7.0_f64 / 4608.0_f64 * t8289 - 7.0_f64 / 2304.0_f64 * t8293 - 7.0_f64 / 576.0_f64 * t8314 - t10777 - t10779 * t10782 / 512.0_f64 + t3626 * t10786 / 512.0_f64 - t3626 * t10790 / 384.0_f64 + t2173 * t10795 / 384.0_f64 + t2173 * t10799 / 768.0_f64 - t10803 + t2173 * t10805 / 768.0_f64 - t2173 * t10809 / 3072.0_f64 - 5.0_f64 / 768.0_f64 * t2173 * t10813;
    t10816
}
