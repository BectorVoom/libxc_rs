//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 911/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk911<F: Float>(t1648: F, t4992: F, t4995: F, t1678: F, t1773: F, t184: F, t199: F, t5130: F, t17128: F, t17133: F, t17138: F, t17141: F, t17144: F, t17147: F, t17150: F) -> (F, F, F, F, F) {
    let t17151 = t1648 * t4992;
    let t17152 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t17151;
    let t17153 = t1648 * t4995;
    let t17154 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t17153;
    let t17158 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t1678 * t1773 * t184 * t199;
    let t17159 = t1648 * t5130;
    let t17160 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t17159;
    let t17161 = -t17128 + t17133 + t17138 - t17141 - t17144 - t17147 + t17150 - t17152 - t17154 + t17158 - t17160;
    (t17152, t17154, t17158, t17160, t17161)
}
