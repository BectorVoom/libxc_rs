//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 594/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk594<F: Float>(t1517: F, t5987: F, t833: F, t509: F, t5867: F, t1153: F, t2429: F, t368: F, t4213: F, t4214: F, t4217: F, t5133: F, t5966: F, t5969: F, t5973: F, t5977: F, t5981: F, t5985: F, t86: F) -> (F, F, F) {
    let t5989 = t1517 * t5987 * t833;
    let t5992 = t509 * t5867;
    let t5996 = t4213 - 0.17687407407407407407e-1 * t4214 - 0.26531111111111111111e-1 * t4217 - 0.17687407407407407407e-1 * t5966 - 0.44218518518518518518e-1 * t5133 * t5969 - 0.26531111111111111111e-1 * t1153 * t5973 + 0.53062222222222222222e-1 * t5133 * t5977 + 0.53062222222222222222e-1 * t2429 * t5981 - 0.26531111111111111111e-1 * t5985 - 0.26531111111111111111e-1 * t1153 * t5989 - 0.39796666666666666666e-1 * t86 * t368 * t5992;
    (t5989, t5992, t5996)
}
