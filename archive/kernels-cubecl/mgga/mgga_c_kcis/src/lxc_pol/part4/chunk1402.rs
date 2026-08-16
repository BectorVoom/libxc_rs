//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1402/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1402<F: Float>(t18217: F, t6176: F, t1369: F, t2470: F, t6164: F, t1599: F, t12615: F, t12664: F, t18184: F, t18188: F, t18192: F, t18197: F, t18201: F, t18205: F, t18213: F, t4435: F, t4439: F, t4442: F, t4451: F, t6141: F) -> F {
    let t18218 = t6176 * t18217;
    let t18221 = t2470 * t1369;
    let t18222 = t18221 * t6164;
    let t18223 = t1599 * t18222;
    let t18225 = F::cast_from(7.0_f64) / F::cast_from(1296.0_f64) * t4439 * t18184 - t4439 * t18188 / F::cast_from(108.0_f64) + t18192 * t4442 / F::cast_from(108.0_f64) + t1599 * t18197 / F::cast_from(48.0_f64) + t1599 * t18201 / F::cast_from(96.0_f64) - t18205 - t6141 * t4451 / F::cast_from(216.0_f64) - t6141 * t4435 / F::cast_from(162.0_f64) + t18213 - t12615 / F::cast_from(576.0_f64) + t12664 / F::cast_from(288.0_f64) - t1599 * t18218 / F::cast_from(32.0_f64) + F::cast_from(7.0_f64) / F::cast_from(864.0_f64) * t18223;
    t18225
}
