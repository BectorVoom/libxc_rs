//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 897/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk897<F: Float>(t1873: F, t3957: F, t1353: F, t1872: F, t800: F, t124: F, t5591: F, t3938: F, t5674: F, t3936: F, t1399: F, t5673: F, t125: F, t1868: F, t1370: F, t3934: F, t3944: F, t3950: F, t3953: F, t3958: F, t3967: F, t3976: F, t3982: F, t3987: F, t3990: F, t3996: F) -> (F, F, F, F, F, F, F, F) {
    let t5681 = t3957 * t1873;
    let t5686 = t800 * t1872 * t1353;
    let t5689 = t124 * t5591;
    let t5690 = t800 * t5689;
    let t5696 = t5674 * t3938;
    let t5697 = t3936 * t5696;
    let t5700 = t5674 * t1399;
    let t5701 = t5673 * t5700;
    let t5704 = t125 * t1868;
    let t5705 = t5704 * t1399;
    let t5706 = t3936 * t5705;
    let t5709 = 7.0 / 144.0 * t5681 + 0.28582678745379824648e-4 * t3953 - t3976 + t3987 + 7.0 / 144.0 * t3958 + t3944 * t5686 / 16.0 + t3967 - t1370 * t5690 / 48.0 - 0.50820002809285328224e-4 * t3982 + 0.40015750243531754508e-2 * t3990 + 0.71456696863449561619e-5 * t3996 + 0.85748036236139473944e-3 * t3934 * t5697 - 0.21437009059034868486e-3 * t3934 * t5701 + 0.85748036236139473944e-3 * t3934 * t5706 + t3950;
    (t5686, t5689, t5690, t5697, t5701, t5704, t5706, t5709)
}
