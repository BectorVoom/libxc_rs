//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1330/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1330(t106: f64, t1411: f64, t14472: f64, t14479: f64, t14488: f64, t17092: f64, t17096: f64, t17245: f64, t25278: f64, t2694: f64, t33492: f64, t335: f64, t3853: f64, t3860: f64, t3861: f64, t43210: f64, t4990: f64, t5049: f64, t51189: f64, t56717: f64, t56764: f64, t56800: f64, t56831: f64, t56865: f64, t56891: f64, t56931: f64, t57561: f64, t57585: f64, t57592: f64, t57657: f64, t57708: f64, t57756: f64, t57813: f64, t57852: f64, t57897: f64, t57943: f64, t57988: f64, t908: f64) -> f64 {
    let t57995 = 0.27818116767324025134e1_f64 * t106 * (t56717 + t56764 + t56800 + t56831 + t56865 + t56891 + t56931 + t57561) * t335 - 0.11127246706929610054e2_f64 * t106 * t51189 * t1411 + 0.33381740120788830161e2_f64 * t106 * t43210 * t4990 - 0.1669087006039441508e2_f64 * t106 * t14472 * t5049 - 0.66763480241577660323e2_f64 * t106 * t33492 * t17092 + 0.66763480241577660323e2_f64 * t14479 * t17096 - 0.11127246706929610054e2_f64 * t106 * t3853 * t17245 + 0.6676348024157766032e2_f64 * t106 * t25278 * t57585 - 0.10014522036236649048e3_f64 * t3860 * t14488 * t5049 + 0.16690870060394415081e2_f64 * t106 * t2694 * t57592 + 0.22254493413859220108e2_f64 * t3860 * t3861 * t17245 - 0.27818116767324025134e1_f64 * t106 * t908 * (t57657 + t57708 + t57756 + t57813 + t57852 + t57897 + t57943 + t57988);
    t57995
}
