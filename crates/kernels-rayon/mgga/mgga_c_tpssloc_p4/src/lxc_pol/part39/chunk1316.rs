//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1316/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1316(t2331: f64, t2585: f64, t2: f64, t666: f64, t1851: f64, t8217: f64, t2205: f64, t2319: f64, t1268: f64, t12725: f64, t12734: f64, t12739: f64, t12823: f64, t1849: f64, t19456: f64, t2200: f64, t2202: f64, t26114: f64, t26117: f64, t30035: f64, t30071: f64, t30266: f64, t30269: f64, t30272: f64, t30330: f64, t4028: f64, t4034: f64, t5113: f64, t55934: f64, t55962: f64, t8176: f64, t8190: f64, t8194: f64, t8274: f64, t8278: f64, t8280: f64, t90370: f64, t90375: f64, t9348: f64) -> (f64, f64, f64, f64, f64) {
    let t110601 = t2585 * t2331;
    let t110602 = t2 * t666;
    let t110919 = 2.0_f64 * t1851 * t8217;
    let t110926 = t2205 * t2319;
    let t110972 = 2.0_f64 * t1268 * t1849 * t30071 - 4.0_f64 * t12725 * t8176 + 4.0_f64 * t12725 * t8194 - 4.0_f64 * t12734 * t8274 + 2.0_f64 * t12739 * t8280 - 2.0_f64 * t12823 * t8274 - 4.0_f64 * t19456 * t8190 + 4.0_f64 * t19456 * t8194 - 4.0_f64 * t2200 * t90370 + 4.0_f64 * t2202 * t55934 + 2.0_f64 * t2202 * t55962 + 2.0_f64 * t2202 * t90375 - 4.0_f64 * t26114 * t8176 + 4.0_f64 * t26117 * t8194 + 2.0_f64 * t30035 * t4028 + 4.0_f64 * t30266 * t5113 + 4.0_f64 * t30269 * t5113 - 4.0_f64 * t30272 * t4034 + 4.0_f64 * t30330 * t5113 + 2.0_f64 * t8278 * t9348;
    (t110601, t110602, t110919, t110926, t110972)
}
