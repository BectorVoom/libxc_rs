//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1397/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1397<F: Float>(t1220: F, t3289: F, t7274: F, t3274: F, t9236: F, t11786: F, t1188: F, t1221: F, t26115: F, t26122: F, t26150: F, t26152: F, t26156: F, t277: F, t27831: F, t3245: F, t3290: F, t4281: F, t4282: F, t4289: F, t4290: F, t8410: F, t914: F, t9244: F, t95: F) -> F {
    let t27837 = t1220 * t7274 * t3289;
    let t27839 = t3274 * t9236;
    let t27841 = -F::cast_from(4.0_f64) * t1220 * t914 * t1221 * t26115 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t11786 * t9244 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4281 * t3245 * t4282 * t26122 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4281 * t4289 * t4290 * t26122 - F::cast_from(2.0_f64) * t8410 * t3290 + F::cast_from(0.25844881434903430496e-2_f64) * t95 * t277 * t27831 * t1188 - t26150 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t27837 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t27839 + t26152 - t26156;
    t27841
}
