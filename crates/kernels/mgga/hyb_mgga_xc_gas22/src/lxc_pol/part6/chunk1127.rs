//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1127/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1127<F: Float>(t2601: F, t4323: F, t1005: F, t1006: F, t11003: F, t10898: F, t10913: F, t6969: F, t7183: F, t9008: F, t9037: F, t7176: F, t9264: F) -> (F, F, F, F, F) {
    let t11139 = t4323 * t2601;
    let t11140 = t11139 * t1005;
    let t11149 = t11003 * t1006;
    let t11159 = -t7183 + F::cast_from(0.22831111111111111111e-1_f64) * t6969 + F::cast_from(0.45662222222222222221e-1_f64) * t9008 - t9037 - F::cast_from(0.17123333333333333333e-1_f64) * t10898 + F::new(0.5137e-1) * t10913;
    let t11166 = -t7176 + F::cast_from(0.23744444444444444444e-1_f64) * t6969 + F::cast_from(0.47488888888888888888e-1_f64) * t9008 - t9264 - F::cast_from(0.17808333333333333333e-1_f64) * t10898 + F::new(0.53425e-1) * t10913;
    (t11139, t11140, t11149, t11159, t11166)
}
