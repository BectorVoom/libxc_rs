//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3181/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3181<F: Float>(t12364: F, t16840: F, t56176: F, t56183: F, t43830: F, t43832: F, t45232: F, t56151: F, t56155: F, t56159: F, t56163: F, t56167: F, t56174: F, t56181: F, t56185: F, t56187: F, t56189: F, t56194: F, t56198: F, t56203: F, t56207: F, t56209: F) -> (F, F) {
    let t58598 = F::cast_from(6.0_f64) * t16840 * t12364;
    let t58607 = F::cast_from(0.1522074074074074074e-1_f64) * t56176;
    let t58609 = F::cast_from(0.4566222222222222222e-1_f64) * t56183;
    let t58618 = t45232 - F::cast_from(0.41095999999999999999e0_f64) * t56151 + F::cast_from(0.10274e0_f64) * t56155 + F::cast_from(0.30822e0_f64) * t56159 + F::cast_from(0.34246666666666666666e-1_f64) * t56163 + F::cast_from(0.41096e0_f64) * t56167 - F::cast_from(0.34246666666666666665e-1_f64) * t43830 + F::cast_from(0.11415555555555555555e-1_f64) * t43832 - F::cast_from(0.50735802469135802467e-1_f64) * t56174 - t58607 + F::cast_from(0.2283111111111111111e0_f64) * t56181 + t58609 - F::cast_from(0.68493333333333333331e-1_f64) * t56185 - F::cast_from(0.34246666666666666665e-1_f64) * t56187 - F::cast_from(0.10274e0_f64) * t56189 - F::cast_from(0.34246666666666666665e-1_f64) * t56194 - F::cast_from(0.34246666666666666665e-1_f64) * t56198 - F::cast_from(0.20547999999999999999e0_f64) * t56203 - F::cast_from(0.11415555555555555555e-1_f64) * t56207 + F::cast_from(0.2283111111111111111e-1_f64) * t56209;
    (t58598, t58618)
}
