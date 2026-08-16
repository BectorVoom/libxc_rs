//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 837/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk837<F: Float>(t16247: F, t87: F, t40: F, t12971: F, t9392: F, t9404: F, t1256: F, t12987: F, t12990: F, t1310: F, t1320: F, t1879: F, t4611: F, t4759: F, t6356: F, t6571: F, t6604: F, t6619: F) -> (F, F, F, F, F, F) {
    let t16248 = t16247 * t87;
    let t16249 = t40 * t16248;
    let t16251 = F::cast_from(12.0_f64) * t12971;
    let t16252 = F::cast_from(0.17544670192365612213e1_f64) * t9392;
    let t16257 = F::cast_from(0.51947267698127589899e2_f64) * t9404;
    let t16262 = -t6356 - t16251 - t6571 + t6604 - t6619 - t16252 - F::cast_from(7.0_f64) / F::cast_from(2.0_f64) * t12987 + F::cast_from(0.23260393291413087447e-1_f64) * t1879 * t12990 * t1256 - t16257 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t4611 * t1320 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1310 * t4759;
    (t16248, t16249, t16251, t16252, t16257, t16262)
}
