//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1445/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1445(t106: f64, t1147: f64, t1550: f64, t15706: f64, t15713: f64, t15722: f64, t17960: f64, t17964: f64, t18174: f64, t27277: f64, t3170: f64, t34319: f64, t4403: f64, t4410: f64, t4411: f64, t46413: f64, t470: f64, t5351: f64, t5430: f64, t54641: f64, t58970: f64, t59028: f64, t59075: f64, t59482: f64, t59531: f64, t59575: f64, t59611: f64, t59643: f64, t59667: f64, t59674: f64, t59719: f64, t59752: f64, t59788: f64, t59804: f64, t59835: f64, t59880: f64, t59916: f64, t59963: f64, t60002: f64, t60031: f64, t60065: f64, t60103: f64, t60135: f64, t60168: f64, t60199: f64, t60235: f64) -> f64 {
    let t60243 = 0.27818116767324025134e1_f64 * t106 * (t58970 + t59028 + t59075 + t59482 + t59531 + t59575 + t59611 + t59643) * t470 - 0.11127246706929610054e2_f64 * t106 * t54641 * t1550 + 0.33381740120788830161e2_f64 * t106 * t46413 * t5351 - 0.1669087006039441508e2_f64 * t106 * t15706 * t5430 - 0.66763480241577660323e2_f64 * t106 * t34319 * t17960 + 0.66763480241577660323e2_f64 * t15713 * t17964 - 0.11127246706929610054e2_f64 * t106 * t4403 * t18174 + 0.6676348024157766032e2_f64 * t106 * t27277 * t59667 - 0.10014522036236649048e3_f64 * t4410 * t15722 * t5430 + 0.16690870060394415081e2_f64 * t106 * t3170 * t59674 + 0.22254493413859220108e2_f64 * t4410 * t4411 * t18174 - 0.27818116767324025134e1_f64 * t106 * t1147 * (t59719 + t59752 + t59788 + t59804 + t59835 + t59880 + t59916 + t59963 + t60002 + t60031 + t60065 + t60103 + t60135 + t60168 + t60199 + t60235);
    t60243
}
