//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1316/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1316<F: Float>(t20885: F, t20899: F, t1250: F, t482: F, t1042: F, t19680: F, t5268: F, t1247: F, t1261: F, t12910: F, t12956: F, t17339: F, t17396: F, t17505: F, t20858: F, t20864: F, t20868: F, t20876: F, t20880: F, t3708: F, t3711: F, t5299: F, t5354: F, t6619: F, t6625: F) -> (F, F) {
    let t20900 = t20885 + t20899;
    let t20902 = t482 * t20900 * t1250;
    let t20903 = t1042 * t20902;
    let t20906 = t5268 * t19680;
    let t20907 = t1042 * t20906;
    let t20910 = t17339 + F::cast_from(0.42874018118069736972e-3_f64) * t12910 * t20858 + F::cast_from(0.22866142996303859718e-2_f64) * t17396 * t5354 + F::cast_from(0.47637797908966374414e-3_f64) * t1261 * t20864 + F::cast_from(0.14291339372689912324e-2_f64) * t1261 * t20868 - F::cast_from(0.15244095330869239812e-2_f64) * t17505 * t5299 + F::cast_from(0.28582678745379824648e-3_f64) * t12956 * t6619 + F::cast_from(0.28582678745379824648e-3_f64) * t3711 * t20876 + F::cast_from(0.28582678745379824648e-3_f64) * t3711 * t20880 + F::cast_from(0.21437009059034868486e-3_f64) * t3708 * t6625 + F::cast_from(0.21437009059034868486e-3_f64) * t1247 * t20903 - F::cast_from(0.28582678745379824648e-3_f64) * t1261 * t20907;
    (t20900, t20910)
}
