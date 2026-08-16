//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1248/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1248(t1020: f64, t26671: f64, t28907: f64, t100162: f64, t100257: f64, t100355: f64, t100360: f64, t100370: f64, t100373: f64, t15231: f64, t19399: f64, t26960: f64, t27070: f64, t28123: f64, t29127: f64, t7772: f64, t92657: f64, t95828: f64) -> (f64, f64) {
    let t100378 = t1020 * t26671 * t28907;
    let t100380 = 0.30952962962962962962e-2_f64 * t95828 + 0.51015085286458333333e-3_f64 * t7772 * t100162 + 0.23214722222222222221e-2_f64 * t100355 - 0.2782641015625e-3_f64 * t7772 * t100257 - 0.30945286961263020834e-5_f64 * t92657 * t100360 - 0.23168402777777777778e-3_f64 * t26960 * t100360 - 0.61782407407407407408e-3_f64 * t26960 * t15231 * t28123 * t19399 - 0.15476481481481481481e-2_f64 * t100370 - 0.23168402777777777778e-3_f64 * t100373 + 0.46377350260416666667e-4_f64 * t27070 * t29127 - 0.30952962962962962963e-2_f64 * t100378;
    (t100378, t100380)
}
