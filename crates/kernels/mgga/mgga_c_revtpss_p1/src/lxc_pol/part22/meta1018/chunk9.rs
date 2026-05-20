//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3530/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3530<F: Float>(t11675: F, t11703: F, t11933: F, t15585: F, t15689: F, t19501: F, t19636: F, t19641: F, t19731: F, t19838: F, t3092: F, t372: F, t42216: F, t42675: F, t42765: F, t42926: F, t42929: F, t42932: F, t43069: F, t43139: F, t43244: F, t4579: F, t4823: F, t4892: F, t4900: F, t54733: F, t55209: F, t6268: F, t905: F) -> F {
    let t66925 = -F::cast_from(0.95275595817932748826e-4_f64) * t42926 - F::cast_from(0.95275595817932748826e-4_f64) * t42929 + F::cast_from(0.47637797908966374413e-4_f64) * t42932 + F::cast_from(0.11433071498151929859e-2_f64) * t15689 * t55209 * t4900 * t4579 + F::cast_from(0.11433071498151929859e-2_f64) * t43069 * t372 * t4823 * t905 * t15585 + F::cast_from(0.45732285992607719436e-2_f64) * t11933 * t19838 + F::cast_from(0.10162730220579493208e-2_f64) * t54733 + F::cast_from(0.28582678745379824648e-3_f64) * t43244 * t6268 + F::cast_from(0.57165357490759649296e-3_f64) * t11675 * t19731 + F::cast_from(0.91464571985215438873e-2_f64) * t42765 * t19636 - F::cast_from(0.45732285992607719436e-2_f64) * t42675 * t19641 + F::cast_from(0.28582678745379824648e-3_f64) * t4892 * t3092 * t19501 * t43139 + F::cast_from(0.47637797908966374413e-3_f64) * t4892 * t11703 * t19501 * t42216;
    t66925
}
