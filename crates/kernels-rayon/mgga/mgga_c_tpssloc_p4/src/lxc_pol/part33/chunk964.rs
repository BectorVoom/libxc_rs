//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 964/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk964(t21013: f64, t235: f64, t20986: f64, t4282: f64, t4295: f64, t5612: f64, t1499: f64, t1523: f64, t1525: f64, t16673: f64, t20806: f64, t20854: f64, t20858: f64, t20862: f64, t20867: f64, t20871: f64, t20873: f64, t20876: f64, t20937: f64, t226: f64, t255: f64, t4166: f64, t4281: f64, t4291: f64, t5575: f64, t5645: f64, t5648: f64, t5651: f64, t5653: f64, t5655: f64, t812: f64) -> (f64, f64) {
    let t21014 = t235 * t21013;
    let t21025 = t4282 * t20986;
    let t21028 = t4295 * t5612;
    let t21033 = 3.0_f64 * t1499 * t5655 - 3.0_f64 * t1523 * t16673 + 3.0_f64 * t1525 * t5575 - 3.0_f64 * t20806 * t812 - t20854 * t812 - 6.0_f64 * t20858 * t812 + 6.0_f64 * t20862 * t812 + 6.0_f64 * t20867 * t812 - t20871 * t812 - 3.0_f64 * t20873 * t4291 - 3.0_f64 * t20876 * t812 + t20937 * t255 + t21014 * t226 + 6.0_f64 * t21025 * t4281 - 3.0_f64 * t21028 * t812 + 6.0_f64 * t4166 * t5645 - 6.0_f64 * t4166 * t5648 - 3.0_f64 * t4166 * t5651 - 3.0_f64 * t4166 * t5653;
    (t21025, t21033)
}
