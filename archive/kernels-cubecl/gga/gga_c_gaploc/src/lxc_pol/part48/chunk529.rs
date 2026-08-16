//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 529/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk529<F: Float>(t10163: F, t9074: F, t123: F, t7887: F, t2326: F, t158: F, t3338: F, t488: F, t3351: F, t484: F, t2854: F, t6509: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10164 = t9074 * t10163;
    let t10165 = F::cast_from(0.11856252764865062333e-2_f64) * t10164;
    let t10166 = t7887 * t123;
    let t10167 = t10166 * t2326;
    let t10168 = t9074 * t10167;
    let t10169 = F::cast_from(0.35568758294595186999e-2_f64) * t10168;
    let t10170 = t158 * t3338;
    let t10171 = t10170 * t123;
    let t10172 = t10171 * t488;
    let t10175 = t484 * t3351;
    let t10176 = F::cast_from(0.15808337019820083111e-2_f64) * t10175;
    let t10177 = t2854 * t6509;
    (t10164, t10165, t10166, t10168, t10169, t10170, t10171, t10172, t10175, t10176, t10177)
}
