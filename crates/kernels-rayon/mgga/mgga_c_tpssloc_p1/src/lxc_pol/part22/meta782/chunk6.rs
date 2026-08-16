//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2678/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2678(t16224: f64, t16225: f64, t16305: f64, t16311: f64, t5246: f64, t5250: f64, t54013: f64, t54199: f64, t56927: f64, t56933: f64, t56935: f64, t56937: f64, t56946: f64, t56953: f64, t56959: f64, t56961: f64, t56963: f64, t56993: f64, t57172: f64, t6388: f64, t74415: f64) -> f64 {
    let t74655 = -119.0_f64 / 4608.0_f64 * t56927 + 7.0_f64 / 1536.0_f64 * t56933 - 7.0_f64 / 384.0_f64 * t56935 - 35.0_f64 / 384.0_f64 * t56937 - t54199 + 35.0_f64 / 24.0_f64 * t56946 - 35.0_f64 / 72.0_f64 * t56953 - 7.0_f64 / 192.0_f64 * t56959 - 7.0_f64 / 192.0_f64 * t56961 - 7.0_f64 / 192.0_f64 * t56963 + 3.0_f64 / 512.0_f64 * t5246 * t54013 * t74415 * t5250 + 5.0_f64 / 128.0_f64 * t5246 * t16224 * t16311 * t57172 - 3.0_f64 / 128.0_f64 * t5246 * t16305 * t6388 * t16225 + 119.0_f64 / 576.0_f64 * t56993;
    t74655
}
