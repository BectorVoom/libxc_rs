//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3151/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3151(t58084: f64, t58105: f64, t1139: f64, t56176: f64, t43828: f64, t43830: f64, t43832: f64, t43911: f64, t56174: f64, t56181: f64, t58055: f64, t58057: f64, t58060: f64, t58063: f64) -> (f64, f64, f64) {
    let t58106 = t58084 + t58105;
    let t58107 = t1139 * t58106;
    let t58114 = 0.45908888888888888888e0_f64 * t56176;
    let t58116 = 0.94674375e0_f64 * t58055 + 0.31558125e0_f64 * t58057 - 0.6618234375e1_f64 * t58060 + 0.2366859375e0_f64 * t58063 + 0.6311625e0_f64 * t58107 - 0.41678000000000000001e0_f64 * t43828 - 0.103295e1_f64 * t43830 + 0.34431666666666666666e0_f64 * t43832 - 0.11577222222222222222e0_f64 * t43911 - 0.15302962962962962963e1_f64 * t56174 - t58114 + 0.68863333333333333334e1_f64 * t56181;
    (t58106, t58107, t58116)
}
