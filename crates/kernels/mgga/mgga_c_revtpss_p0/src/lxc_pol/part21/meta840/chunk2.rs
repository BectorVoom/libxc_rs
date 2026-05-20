//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3151/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3151<F: Float>(t58084: F, t58105: F, t1139: F, t56176: F, t43828: F, t43830: F, t43832: F, t43911: F, t56174: F, t56181: F, t58055: F, t58057: F, t58060: F, t58063: F) -> (F, F, F) {
    let t58106 = t58084 + t58105;
    let t58107 = t1139 * t58106;
    let t58114 = F::cast_from(0.45908888888888888888e0_f64) * t56176;
    let t58116 = F::new(0.94674375e0) * t58055 + F::new(0.31558125e0) * t58057 - F::cast_from(0.6618234375e1_f64) * t58060 + F::cast_from(0.2366859375e0_f64) * t58063 + F::new(0.6311625e0) * t58107 - F::cast_from(0.41678000000000000001e0_f64) * t43828 - F::new(0.103295e1) * t43830 + F::cast_from(0.34431666666666666666e0_f64) * t43832 - F::cast_from(0.11577222222222222222e0_f64) * t43911 - F::cast_from(0.15302962962962962963e1_f64) * t56174 - t58114 + F::cast_from(0.68863333333333333334e1_f64) * t56181;
    (t58106, t58107, t58116)
}
