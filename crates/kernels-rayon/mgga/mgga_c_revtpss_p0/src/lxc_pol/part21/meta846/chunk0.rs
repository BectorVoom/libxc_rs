//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3165/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3165(t43828: f64, t43830: f64, t43832: f64, t43911: f64, t56174: f64, t56176: f64, t56181: f64, t58055: f64, t58057: f64, t58060: f64, t58063: f64, t58107: f64) -> f64 {
    let t58386 = 0.247573125e0_f64 * t58055 + 0.82524375e-1_f64 * t58057 - 0.485484375e1_f64 * t58060 + 0.6189328125e-1_f64 * t58063 + 0.16504875e0_f64 * t58107 - 0.33114e0_f64 * t43828 - 0.60385000000000000002e0_f64 * t43830 + 0.20128333333333333334e0_f64 * t43832 - 0.91983333333333333335e-1_f64 * t43911 - 0.89459259259259259259e0_f64 * t56174 - 0.26837777777777777778e0_f64 * t56176 + 0.40256666666666666666e1_f64 * t56181;
    t58386
}
