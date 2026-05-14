//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1110/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1110<F: Float>(t1020: F, t19638: F, t27763: F, t20314: F, t5310: F, t922: F, t19702: F, t3200: F, t92808: F, t15573: F, t29147: F, t7788: F, t26671: F, t28907: F, t100162: F, t100257: F, t15231: F, t19399: F, t26960: F, t27070: F, t28123: F, t29127: F, t7772: F, t92657: F, t95828: F) -> (F, F, F, F, F) {
    let t100355 = t1020 * t27763 * t19638;
    let t100360 = t5310 * t20314 * t922;
    let t100370 = t3200 * t92808 * t19702;
    let t100373 = t7788 * t15573 * t29147;
    let t100378 = t1020 * t26671 * t28907;
    let t100380 = 0.30952962962962962962e-2 * t95828 + 0.51015085286458333333e-3 * t7772 * t100162 + 0.23214722222222222221e-2 * t100355 - 0.2782641015625e-3 * t7772 * t100257 - 0.30945286961263020834e-5 * t92657 * t100360 - 0.23168402777777777778e-3 * t26960 * t100360 - 0.61782407407407407408e-3 * t26960 * t15231 * t28123 * t19399 - 0.15476481481481481481e-2 * t100370 - 0.23168402777777777778e-3 * t100373 + 0.46377350260416666667e-4 * t27070 * t29127 - 0.30952962962962962963e-2 * t100378;
    (t100355, t100360, t100370, t100378, t100380)
}
