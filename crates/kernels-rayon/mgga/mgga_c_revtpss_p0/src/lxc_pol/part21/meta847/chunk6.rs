//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3181/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3181(t12364: f64, t16840: f64, t56176: f64, t56183: f64, t43830: f64, t43832: f64, t45232: f64, t56151: f64, t56155: f64, t56159: f64, t56163: f64, t56167: f64, t56174: f64, t56181: f64, t56185: f64, t56187: f64, t56189: f64, t56194: f64, t56198: f64, t56203: f64, t56207: f64, t56209: f64) -> (f64, f64) {
    let t58598 = 6.0_f64 * t16840 * t12364;
    let t58607 = 0.1522074074074074074e-1_f64 * t56176;
    let t58609 = 0.4566222222222222222e-1_f64 * t56183;
    let t58618 = t45232 - 0.41095999999999999999e0_f64 * t56151 + 0.10274e0_f64 * t56155 + 0.30822e0_f64 * t56159 + 0.34246666666666666666e-1_f64 * t56163 + 0.41096e0_f64 * t56167 - 0.34246666666666666665e-1_f64 * t43830 + 0.11415555555555555555e-1_f64 * t43832 - 0.50735802469135802467e-1_f64 * t56174 - t58607 + 0.2283111111111111111e0_f64 * t56181 + t58609 - 0.68493333333333333331e-1_f64 * t56185 - 0.34246666666666666665e-1_f64 * t56187 - 0.10274e0_f64 * t56189 - 0.34246666666666666665e-1_f64 * t56194 - 0.34246666666666666665e-1_f64 * t56198 - 0.20547999999999999999e0_f64 * t56203 - 0.11415555555555555555e-1_f64 * t56207 + 0.2283111111111111111e-1_f64 * t56209;
    (t58598, t58618)
}
