//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1440/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1440(t19: f64, t59022: f64, t1179: f64, t123: f64, t15653: f64, t17979: f64, t17994: f64, t18019: f64, t18145: f64, t27215: f64, t27651: f64, t27706: f64, t27712: f64, t27786: f64, t3107: f64, t3217: f64, t35653: f64, t438: f64, t4450: f64, t450: f64, t4501: f64, t458: f64, t45885: f64, t55493: f64, t55496: f64, t55498: f64, t58941: f64, t59030: f64, t59511: f64, t60060: f64, t8973: f64) -> f64 {
    let t60075 = t59022 * t19;
    let t60103 = -0.38640729216933594422e6_f64 * t27215 * t450 * t60060 * t438 - 0.1039653020352937208e2_f64 * t45885 - 0.27022098409157095356e7_f64 * t27786 * t458 * t59030 * t19 + 0.10508593825783314861e7_f64 * t27706 * t458 * t60075 * t3107 - 0.75061384469880820436e5_f64 * t27712 * t458 * t60075 * t438 - 0.13186481011862155443e4_f64 * t3217 * t458 * t58941 * t123 * t438 - 0.61944912485988186948e2_f64 * t8973 * t15653 * t18019 + 0.26631068404529536697e4_f64 * t55493 + 0.24727214904288022343e1_f64 * t4450 * t18145 + 0.35163949364965747848e4_f64 * t55496 - 0.41212024840480037237e0_f64 * t55498 - 0.21304854723623629356e5_f64 * t35653 * t17979 - 0.1209136907000740735e0_f64 * t1179 * t59511 - 0.12117441361606500412e2_f64 * t4501 * t17994 - t27651;
    t60103
}
