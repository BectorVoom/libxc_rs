//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1212/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1212<F: Float>(t15562: F, t5329: F, t3530: F, t5336: F, t1262: F, t25: F, t287: F, t5331: F, t1251: F, t10996: F, t11086: F, t15531: F, t15535: F, t15541: F, t15547: F, t15549: F, t15555: F, t15558: F, t1847: F, t3490: F, t3514: F, t5316: F, t5326: F, t5332: F) -> F {
    let t15563 = t5329 * t15562;
    let t15568 = t3530 * t5336;
    let t15569 = t15568 * t1262;
    let t15570 = t5329 * t15569;
    let t15573 = t25 * t287;
    let t15574 = t15573 * t5331;
    let t15576 = t1251 * t15574 / F::cast_from(144.0_f64);
    let t15577 = -t3514 * t15531 / F::cast_from(432.0_f64) - t3514 * t15535 / F::cast_from(72.0_f64) + t11086 * t5326 / F::cast_from(108.0_f64) + t3514 * t15541 / F::cast_from(288.0_f64) + F::cast_from(11.0_f64) / F::cast_from(648.0_f64) * t10996 * t1847 - t15547 - t15549 / F::cast_from(2592.0_f64) - t3490 * t5316 / F::cast_from(54.0_f64) - F::cast_from(7.0_f64) / F::cast_from(864.0_f64) * t15555 + t1251 * t15558 / F::cast_from(96.0_f64) - t1251 * t15563 / F::cast_from(32.0_f64) - t3490 * t5332 / F::cast_from(18.0_f64) + t1251 * t15570 / F::cast_from(48.0_f64) + t15576;
    t15577
}
