//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 881/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk881<F: Float>(t2029: F, t238: F, t86: F, t1979: F, t531: F, t1517: F, t833: F, t509: F, t5867: F, t1153: F, t2429: F, t368: F, t4213: F, t4214: F, t4217: F, t5133: F, t5966: F, t5969: F, t5973: F, t5977: F, t5981: F) -> (F, F, F, F) {
    let t5985 = t86 * t238 * t2029;
    let t5987 = t1979 * t531;
    let t5989 = t1517 * t5987 * t833;
    let t5992 = t509 * t5867;
    let t5996 = t4213 - F::new(0.17687407407407407407e-1) * t4214 - F::new(0.26531111111111111111e-1) * t4217 - F::new(0.17687407407407407407e-1) * t5966 - F::new(0.44218518518518518518e-1) * t5133 * t5969 - F::new(0.26531111111111111111e-1) * t1153 * t5973 + F::new(0.53062222222222222222e-1) * t5133 * t5977 + F::new(0.53062222222222222222e-1) * t2429 * t5981 - F::new(0.26531111111111111111e-1) * t5985 - F::new(0.26531111111111111111e-1) * t1153 * t5989 - F::new(0.39796666666666666666e-1) * t86 * t368 * t5992;
    (t5987, t5989, t5992, t5996)
}
