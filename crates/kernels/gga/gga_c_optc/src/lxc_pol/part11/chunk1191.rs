//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1191/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1191<F: Float>(t106: F, t1411: F, t14472: F, t14479: F, t14488: F, t17092: F, t17096: F, t17245: F, t25278: F, t2694: F, t33492: F, t335: F, t3853: F, t3860: F, t3861: F, t43210: F, t4990: F, t5049: F, t51189: F, t56717: F, t56764: F, t56800: F, t56831: F, t56865: F, t56891: F, t56931: F, t57561: F, t57585: F, t57592: F, t57657: F, t57708: F, t57756: F, t57813: F, t57852: F, t57897: F, t57943: F, t57988: F, t908: F) -> (F,) {
    let t57995 = 0.27818116767324025134e1 * t106 * (t56717 + t56764 + t56800 + t56831 + t56865 + t56891 + t56931 + t57561) * t335 - 0.11127246706929610054e2 * t106 * t51189 * t1411 + 0.33381740120788830161e2 * t106 * t43210 * t4990 - 0.1669087006039441508e2 * t106 * t14472 * t5049 - 0.66763480241577660323e2 * t106 * t33492 * t17092 + 0.66763480241577660323e2 * t14479 * t17096 - 0.11127246706929610054e2 * t106 * t3853 * t17245 + 0.6676348024157766032e2 * t106 * t25278 * t57585 - 0.10014522036236649048e3 * t3860 * t14488 * t5049 + 0.16690870060394415081e2 * t106 * t2694 * t57592 + 0.22254493413859220108e2 * t3860 * t3861 * t17245 - 0.27818116767324025134e1 * t106 * t908 * (t57657 + t57708 + t57756 + t57813 + t57852 + t57897 + t57943 + t57988);
    (t57995,)
}
