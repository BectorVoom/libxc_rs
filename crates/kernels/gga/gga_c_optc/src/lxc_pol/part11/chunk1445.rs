//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1445/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1445<F: Float>(t106: F, t1147: F, t1550: F, t15706: F, t15713: F, t15722: F, t17960: F, t17964: F, t18174: F, t27277: F, t3170: F, t34319: F, t4403: F, t4410: F, t4411: F, t46413: F, t470: F, t5351: F, t5430: F, t54641: F, t58970: F, t59028: F, t59075: F, t59482: F, t59531: F, t59575: F, t59611: F, t59643: F, t59667: F, t59674: F, t59719: F, t59752: F, t59788: F, t59804: F, t59835: F, t59880: F, t59916: F, t59963: F, t60002: F, t60031: F, t60065: F, t60103: F, t60135: F, t60168: F, t60199: F, t60235: F) -> F {
    let t60243 = F::cast_from(0.27818116767324025134e1_f64) * t106 * (t58970 + t59028 + t59075 + t59482 + t59531 + t59575 + t59611 + t59643) * t470 - F::cast_from(0.11127246706929610054e2_f64) * t106 * t54641 * t1550 + F::cast_from(0.33381740120788830161e2_f64) * t106 * t46413 * t5351 - F::cast_from(0.1669087006039441508e2_f64) * t106 * t15706 * t5430 - F::cast_from(0.66763480241577660323e2_f64) * t106 * t34319 * t17960 + F::cast_from(0.66763480241577660323e2_f64) * t15713 * t17964 - F::cast_from(0.11127246706929610054e2_f64) * t106 * t4403 * t18174 + F::cast_from(0.6676348024157766032e2_f64) * t106 * t27277 * t59667 - F::cast_from(0.10014522036236649048e3_f64) * t4410 * t15722 * t5430 + F::cast_from(0.16690870060394415081e2_f64) * t106 * t3170 * t59674 + F::cast_from(0.22254493413859220108e2_f64) * t4410 * t4411 * t18174 - F::cast_from(0.27818116767324025134e1_f64) * t106 * t1147 * (t59719 + t59752 + t59788 + t59804 + t59835 + t59880 + t59916 + t59963 + t60002 + t60031 + t60065 + t60103 + t60135 + t60168 + t60199 + t60235);
    t60243
}
