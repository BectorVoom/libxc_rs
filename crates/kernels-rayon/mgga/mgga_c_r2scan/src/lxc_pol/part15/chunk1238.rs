//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1238/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1238(t40844: f64, t37066: f64, t23495: f64, t3363: f64, t37029: f64, t37039: f64, t37063: f64, t37076: f64, t40817: f64, t40822: f64, t40825: f64, t40828: f64, t40830: f64, t40833: f64, t40835: f64, t40837: f64, t40839: f64, t40841: f64, t40842: f64) -> f64 {
    let t40845 = 2.0_f64 / 3.0_f64 * t40844;
    let t40846 = 22.0_f64 / 9.0_f64 * t37066;
    let t40848 = t23495 * t3363;
    let t40850 = 3.0_f64 * t40817 + t40822 - 3.0_f64 / 2.0_f64 * t40825 - 3.0_f64 / 4.0_f64 * t40828 + t40830 / 8.0_f64 + 2.0_f64 / 3.0_f64 * t37029 - t40833 / 2.0_f64 - t40835 / 4.0_f64 - t40837 / 8.0_f64 + t40839 + t37039 - t40841 + 3.0_f64 / 4.0_f64 * t40842 + t40845 - t40846 + t37076 + t37063 / 3.0_f64 + t40848 / 2.0_f64;
    t40850
}
