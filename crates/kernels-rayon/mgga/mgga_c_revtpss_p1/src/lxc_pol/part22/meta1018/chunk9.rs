//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3530/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3530(t11675: f64, t11703: f64, t11933: f64, t15585: f64, t15689: f64, t19501: f64, t19636: f64, t19641: f64, t19731: f64, t19838: f64, t3092: f64, t372: f64, t42216: f64, t42675: f64, t42765: f64, t42926: f64, t42929: f64, t42932: f64, t43069: f64, t43139: f64, t43244: f64, t4579: f64, t4823: f64, t4892: f64, t4900: f64, t54733: f64, t55209: f64, t6268: f64, t905: f64) -> f64 {
    let t66925 = -0.95275595817932748826e-4_f64 * t42926 - 0.95275595817932748826e-4_f64 * t42929 + 0.47637797908966374413e-4_f64 * t42932 + 0.11433071498151929859e-2_f64 * t15689 * t55209 * t4900 * t4579 + 0.11433071498151929859e-2_f64 * t43069 * t372 * t4823 * t905 * t15585 + 0.45732285992607719436e-2_f64 * t11933 * t19838 + 0.10162730220579493208e-2_f64 * t54733 + 0.28582678745379824648e-3_f64 * t43244 * t6268 + 0.57165357490759649296e-3_f64 * t11675 * t19731 + 0.91464571985215438873e-2_f64 * t42765 * t19636 - 0.45732285992607719436e-2_f64 * t42675 * t19641 + 0.28582678745379824648e-3_f64 * t4892 * t3092 * t19501 * t43139 + 0.47637797908966374413e-3_f64 * t4892 * t11703 * t19501 * t42216;
    t66925
}
