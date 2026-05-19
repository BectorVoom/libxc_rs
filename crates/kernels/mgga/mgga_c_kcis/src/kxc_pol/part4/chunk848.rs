//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 848/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk848<F: Float>(t3883: F, t5427: F, t26: F, t1330: F, t5441: F, t5477: F, t4714: F, t3795: F, t3868: F, t3880: F, t3881: F, t5469: F, t5472: F, t5475: F, t5479: F, t5514: F, t5516: F, t5557: F, t5559: F, t5562: F) -> (F, F, F, F, F, F, F) {
    let t5564 = t3883 * t5427;
    let t5565 = t26 * t5564;
    let t5567 = t1330 * t5441;
    let t5568 = t26 * t5567;
    let t5570 = t1330 * t5477;
    let t5571 = t4714 * t5570;
    let t5573 = -F::new(0.9494625e0) * t5514 + F::new(0.1898925e1) * t5516 + t3868 + F::cast_from(0.99655555555555555557e-1_f64) * t3795 + F::cast_from(0.99655555555555555557e-1_f64) * t5469 - F::cast_from(0.19931111111111111111e0_f64) * t5472 + F::cast_from(0.59793333333333333334e0_f64) * t5475 + F::cast_from(0.59793333333333333334e0_f64) * t5479 + F::new(0.15358125e0) * t5557 + F::new(0.3071625e0) * t5559 + t3880 + F::cast_from(0.54771111111111111111e-1_f64) * t3881 + F::cast_from(0.54771111111111111111e-1_f64) * t5562 - F::cast_from(0.27385555555555555556e-1_f64) * t5565 + F::cast_from(0.16431333333333333333e0_f64) * t5568 + F::cast_from(0.16431333333333333333e0_f64) * t5571;
    (t5564, t5565, t5567, t5568, t5570, t5571, t5573)
}
