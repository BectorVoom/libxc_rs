//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1440/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1440<F: Float>(t19: F, t59022: F, t1179: F, t123: F, t15653: F, t17979: F, t17994: F, t18019: F, t18145: F, t27215: F, t27651: F, t27706: F, t27712: F, t27786: F, t3107: F, t3217: F, t35653: F, t438: F, t4450: F, t450: F, t4501: F, t458: F, t45885: F, t55493: F, t55496: F, t55498: F, t58941: F, t59030: F, t59511: F, t60060: F, t8973: F) -> F {
    let t60075 = t59022 * t19;
    let t60103 = -F::cast_from(0.38640729216933594422e6_f64) * t27215 * t450 * t60060 * t438 - F::cast_from(0.1039653020352937208e2_f64) * t45885 - F::cast_from(0.27022098409157095356e7_f64) * t27786 * t458 * t59030 * t19 + F::cast_from(0.10508593825783314861e7_f64) * t27706 * t458 * t60075 * t3107 - F::cast_from(0.75061384469880820436e5_f64) * t27712 * t458 * t60075 * t438 - F::cast_from(0.13186481011862155443e4_f64) * t3217 * t458 * t58941 * t123 * t438 - F::cast_from(0.61944912485988186948e2_f64) * t8973 * t15653 * t18019 + F::cast_from(0.26631068404529536697e4_f64) * t55493 + F::cast_from(0.24727214904288022343e1_f64) * t4450 * t18145 + F::cast_from(0.35163949364965747848e4_f64) * t55496 - F::cast_from(0.41212024840480037237e0_f64) * t55498 - F::cast_from(0.21304854723623629356e5_f64) * t35653 * t17979 - F::cast_from(0.1209136907000740735e0_f64) * t1179 * t59511 - F::cast_from(0.12117441361606500412e2_f64) * t4501 * t17994 - t27651;
    t60103
}
