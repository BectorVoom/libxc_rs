//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1269/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1269<F: Float>(t1330: F, t16082: F, t26: F, t1324: F, t16194: F, t494: F, t531: F, t250: F, t3106: F, t11608: F, t11609: F, t16195: F, t16198: F, t16201: F, t16204: F, t16207: F, t16210: F) -> (F, F, F, F) {
    let t16212 = t1330 * t16082;
    let t16213 = t26 * t16212;
    let t16215 = t1324 * t16194;
    let t16217 = t494 * t531;
    let t16219 = t250 * t3106 * t16217;
    let t16221 = F::new(0.1898925e1) * t16195 + F::cast_from(0.16431333333333333333e0_f64) * t16198 - F::cast_from(0.49293999999999999999e0_f64) * t16201 - F::cast_from(0.27385555555555555556e-1_f64) * t16204 - F::cast_from(0.36514074074074074075e-1_f64) * t16207 + F::cast_from(0.10954222222222222222e0_f64) * t16210 + F::cast_from(0.16431333333333333333e0_f64) * t16213 + F::new(0.3071625e0) * t16215 - t11608 - t11609 + F::cast_from(0.16431333333333333333e0_f64) * t16219;
    (t16213, t16215, t16219, t16221)
}
