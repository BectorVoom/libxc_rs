//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1314/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1314<F: Float>(t1368: F, t16857: F, t12159: F, t1938: F, t4001: F, t613: F, t3971: F, t5691: F, t16830: F, t16833: F, t16838: F, t16842: F, t16845: F, t16850: F, t16854: F, t1930: F, t3991: F, t3995: F, t4003: F, t493: F) -> F {
    let t16858 = t1368 * t16857;
    let t16861 = t12159 * t1938 * t4001;
    let t16862 = t613 * t16861;
    let t16866 = t5691 * t3971 / F::cast_from(162.0_f64);
    let t16869 = t16830 * t16833 / F::cast_from(72.0_f64) - t493 * t16838 / F::cast_from(144.0_f64) + t16842 / F::cast_from(432.0_f64) + t16845 - t1930 * t4003 / F::cast_from(18.0_f64) + F::cast_from(7.0_f64) / F::cast_from(432.0_f64) * t16850 + t16854 + t5691 * t3991 / F::cast_from(54.0_f64) - t16858 / F::cast_from(1296.0_f64) - t1368 * t16862 / F::cast_from(16.0_f64) - t16866 - t5691 * t3995 / F::cast_from(108.0_f64);
    t16869
}
