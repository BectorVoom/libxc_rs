//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1694/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1694<F: Float>(t5819: F, t6258: F, t1469: F, t22671: F, t1651: F, t22688: F, t1042: F, t1063: F, t1066: F, t11703: F, t15716: F, t16081: F, t16208: F, t23481: F, t23911: F, t247: F, t3091: F, t3092: F, t3116: F, t3127: F, t43253: F, t4801: F, t4806: F, t4837: F, t65581: F, t65596: F, t78496: F, t78910: F, t78915: F, t78986: F, t88091: F, t88646: F, t88750: F) -> (F, F, F, F) {
    let t88901 = t5819 * t6258;
    let t88916 = t22671 * t1469;
    let t88925 = t22688 * t1651;
    let t88944 = -F::cast_from(0.22866142996303859718e-2_f64) * t78910 - F::cast_from(0.11433071498151929859e-2_f64) * t78915 - F::cast_from(0.14291339372689912324e-2_f64) * t3127 * t1042 * t4806 * t88901 + F::cast_from(0.34299214494455789578e-2_f64) * t16081 * t3092 * t78496 * t43253 * t1469 + F::cast_from(0.17149607247227894789e-2_f64) * t78986 - F::cast_from(0.34299214494455789578e-2_f64) * t4837 * t1042 * t4801 * t88750 - F::cast_from(0.11433071498151929859e-2_f64) * t1063 * t1042 * t4801 * t88916 + F::cast_from(0.95275595817932748828e-3_f64) * t1063 * t1042 * t4806 * t88916 - F::cast_from(0.2540682555144873302e-2_f64) * t3127 * t1042 * t16208 * t88925 - F::cast_from(0.57165357490759649296e-2_f64) * t3091 * t11703 * t23481 * t23911 + F::cast_from(0.28582678745379824648e-3_f64) * t65581 + F::cast_from(0.3811023832717309953e-3_f64) * t65596 - F::cast_from(0.85748036236139473944e-3_f64) * t1063 * t247 * t1066 * t88091 - F::cast_from(0.77173232612525526552e-2_f64) * t15716 * t247 * t3116 * t88646;
    (t88901, t88916, t88925, t88944)
}
