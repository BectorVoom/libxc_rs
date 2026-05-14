//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1066/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1066<F: Float>(t15794: F, t3115: F, t1032: F, t4743: F, t1040: F, t11921: F, t247: F, t4757: F, t4837: F, t1659: F, t3105: F, t1062: F, t4797: F, t1660: F, t3201: F, t1058: F, t4798: F) -> (F, F, F, F, F, F, F) {
    let t15796 = 0.28582678745379824648e-3 * t3115 * t15794;
    let t15816 = t4743 * t1032;
    let t15817 = t15816 * t1040;
    let t15827 = t247 * t11921 * t4757;
    let t15829 = 0.57165357490759649296e-3 * t4837 * t15827;
    let t15830 = t1659 * t3105;
    let t15850 = t4797 * t1062;
    let t15862 = t1660 * t3201;
    let t15865 = 0.28582678745379824648e-3 * t4798 * t1058;
    (t15796, t15817, t15829, t15830, t15850, t15862, t15865)
}
