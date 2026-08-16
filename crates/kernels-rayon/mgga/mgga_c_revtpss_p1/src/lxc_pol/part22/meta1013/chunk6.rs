//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3485/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3485(t11656: f64, t11994: f64, t15707: f64, t16140: f64, t1671: f64, t19895: f64, t43268: f64, t4825: f64, t53359: f64, t53363: f64, t53692: f64, t54739: f64, t6308: f64, t65567: f64, t65570: f64, t65581: f64, t65585: f64, t65589: f64) -> f64 {
    let t65591 = 0.3811023832717309953e-3_f64 * t53359 + 0.19055119163586549765e-3_f64 * t53363 + 0.31758531939310916276e-3_f64 * t65567 - 0.3811023832717309953e-3_f64 * t65570 + 0.57165357490759649296e-3_f64 * t11994 * t19895 - 0.30488190661738479624e-2_f64 * t11656 * t19895 - 0.57165357490759649296e-3_f64 * t53692 * t4825 - 0.57165357490759649296e-3_f64 * t15707 * t16140 + 0.47637797908966374413e-4_f64 * t65581 - 0.45732285992607719436e-2_f64 * t43268 * t6308 + 0.57165357490759649296e-3_f64 * t65585 - 0.45732285992607719436e-2_f64 * t54739 * t1671 + 0.20325460441158986416e-2_f64 * t65589;
    t65591
}
