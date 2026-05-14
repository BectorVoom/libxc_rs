//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1200/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1200<F: Float>(t15935: F, t15936: F, t1042: F, t3173: F, t4879: F, t1063: F, t11802: F, t11814: F, t11818: F, t11994: F, t15917: F, t15922: F, t15926: F, t15932: F, t3115: F, t3120: F, t3164: F, t3188: F, t4803: F, t4808: F, t4825: F, t4902: F) -> (F,) {
    let t15937 = t15935 * t15936;
    let t15938 = t1042 * t15937;
    let t15942 = 0.28582678745379824648e-3 * t4879 * t3173;
    let t15949 = -0.42874018118069736972e-3 * t15917 * t4902 - 0.42874018118069736972e-3 * t3115 * t15922 - 0.42874018118069736972e-3 * t15926 * t3120 + 0.19055119163586549765e-3 * t11802 + 0.15244095330869239812e-2 * t11814 + 0.95275595817932748826e-4 * t11818 - 0.21437009059034868486e-3 * t15932 * t3164 + 0.85748036236139473944e-3 * t1063 * t15938 + t15942 - 0.28582678745379824648e-3 * t11994 * t4825 - 0.57165357490759649296e-3 * t3188 * t4803 + 0.47637797908966374414e-3 * t3188 * t4808;
    (t15949,)
}
