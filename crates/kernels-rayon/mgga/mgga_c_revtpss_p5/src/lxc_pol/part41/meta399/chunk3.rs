//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1360/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1360(t20885: f64, t20899: f64, t1250: f64, t482: f64, t1042: f64, t19680: f64, t5268: f64, t1247: f64, t1261: f64, t12910: f64, t12956: f64, t17339: f64, t17396: f64, t17505: f64, t20858: f64, t20864: f64, t20868: f64, t20876: f64, t20880: f64, t3708: f64, t3711: f64, t5299: f64, t5354: f64, t6619: f64, t6625: f64) -> (f64, f64) {
    let t20900 = t20885 + t20899;
    let t20902 = t482 * t20900 * t1250;
    let t20903 = t1042 * t20902;
    let t20906 = t5268 * t19680;
    let t20907 = t1042 * t20906;
    let t20910 = t17339 + 0.42874018118069736972e-3_f64 * t12910 * t20858 + 0.22866142996303859718e-2_f64 * t17396 * t5354 + 0.47637797908966374414e-3_f64 * t1261 * t20864 + 0.14291339372689912324e-2_f64 * t1261 * t20868 - 0.15244095330869239812e-2_f64 * t17505 * t5299 + 0.28582678745379824648e-3_f64 * t12956 * t6619 + 0.28582678745379824648e-3_f64 * t3711 * t20876 + 0.28582678745379824648e-3_f64 * t3711 * t20880 + 0.21437009059034868486e-3_f64 * t3708 * t6625 + 0.21437009059034868486e-3_f64 * t1247 * t20903 - 0.28582678745379824648e-3_f64 * t1261 * t20907;
    (t20900, t20910)
}
