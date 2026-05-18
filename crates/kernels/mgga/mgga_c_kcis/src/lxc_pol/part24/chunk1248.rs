//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1248/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1248<F: Float>(t1020: F, t26671: F, t28907: F, t100162: F, t100257: F, t100355: F, t100360: F, t100370: F, t100373: F, t15231: F, t19399: F, t26960: F, t27070: F, t28123: F, t29127: F, t7772: F, t92657: F, t95828: F) -> (F, F) {
    let t100378 = t1020 * t26671 * t28907;
    let t100380 = F::new(0.30952962962962962962e-2) * t95828 + F::new(0.51015085286458333333e-3) * t7772 * t100162 + F::new(0.23214722222222222221e-2) * t100355 - F::new(0.2782641015625e-3) * t7772 * t100257 - F::new(0.30945286961263020834e-5) * t92657 * t100360 - F::new(0.23168402777777777778e-3) * t26960 * t100360 - F::new(0.61782407407407407408e-3) * t26960 * t15231 * t28123 * t19399 - F::new(0.15476481481481481481e-2) * t100370 - F::new(0.23168402777777777778e-3) * t100373 + F::new(0.46377350260416666667e-4) * t27070 * t29127 - F::new(0.30952962962962962963e-2) * t100378;
    (t100378, t100380)
}
