//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1602/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1602(t15935: f64, t15936: f64, t1042: f64, t3173: f64, t4879: f64, t1063: f64, t11802: f64, t11814: f64, t11818: f64, t11994: f64, t15917: f64, t15922: f64, t15926: f64, t15932: f64, t3115: f64, t3120: f64, t3164: f64, t3188: f64, t4803: f64, t4808: f64, t4825: f64, t4902: f64) -> (f64, f64) {
    let t15937 = t15935 * t15936;
    let t15938 = t1042 * t15937;
    let t15942 = 0.28582678745379824648e-3_f64 * t4879 * t3173;
    let t15949 = -0.42874018118069736972e-3_f64 * t15917 * t4902 - 0.42874018118069736972e-3_f64 * t3115 * t15922 - 0.42874018118069736972e-3_f64 * t15926 * t3120 + 0.19055119163586549765e-3_f64 * t11802 + 0.15244095330869239812e-2_f64 * t11814 + 0.95275595817932748826e-4_f64 * t11818 - 0.21437009059034868486e-3_f64 * t15932 * t3164 + 0.85748036236139473944e-3_f64 * t1063 * t15938 + t15942 - 0.28582678745379824648e-3_f64 * t11994 * t4825 - 0.57165357490759649296e-3_f64 * t3188 * t4803 + 0.47637797908966374414e-3_f64 * t3188 * t4808;
    (t15938, t15949)
}
