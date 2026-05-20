//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3504/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3504<F: Float>(t1058: F, t19858: F, t15688: F, t16509: F, t1053: F, t11632: F, t11703: F, t15604: F, t15691: F, t1592: F, t15973: F, t16226: F, t16230: F, t19450: F, t19857: F, t225: F, t3117: F, t3133: F, t3151: F, t3155: F, t366: F, t375: F, t42690: F, t4899: F, t53320: F, t53322: F, t53332: F, t53741: F, t53805: F, t53810: F, t53820: F, t6092: F, t60927: F, t65057: F) -> F {
    let t66093 = t19858 * t1058;
    let t66114 = t16509 * t15688;
    let t66127 = -F::cast_from(0.3811023832717309953e-3_f64) * t53805 + F::cast_from(0.21437009059034868486e-3_f64) * t65057 * t225 * t366 * t375 + F::cast_from(0.28582678745379824648e-3_f64) * t66093 - F::cast_from(0.22866142996303859718e-2_f64) * t19857 * t1053 * t375 - t53320 * t53332 * t60927 / F::new(9.0) + F::new(7.0) / F::new(162.0) * t53320 * t53322 * t60927 - F::cast_from(0.42874018118069736972e-3_f64) * t42690 * t3117 * t19450 * t15604 - F::cast_from(0.23818898954483187207e-3_f64) * t4899 * t11703 * t6092 * t15973 + F::cast_from(0.57165357490759649296e-3_f64) * t53810 + F::cast_from(0.76220476654346199061e-3_f64) * t53820 + F::cast_from(0.11433071498151929859e-2_f64) * t66114 * t16230 + F::cast_from(0.57165357490759649296e-3_f64) * t16226 * t15691 * t3155 * t1592 * t3133 + F::cast_from(0.17149607247227894789e-2_f64) * t53741 * t15691 * t11632 * t1592 * t3151;
    t66127
}
