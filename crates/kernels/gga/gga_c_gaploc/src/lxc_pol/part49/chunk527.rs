//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 527/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk527<F: Float>(t550: F, t9198: F, t1365: F, t1063: F, t1358: F, t2268: F, t3138: F, t419: F, t9162: F, t9165: F, t9168: F, t9173: F, t9178: F, t9183: F, t9186: F, t9190: F, t9195: F) -> (F, F) {
    let t9199 = t550 * t9198;
    let t9200 = t1365 * t9199;
    let t9203 = -F::cast_from(0.28455006635676149599e-1_f64) * t419 * t3138 - F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t9162 - F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t9165 + F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t9168 + F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t9173 - F::cast_from(0.17073003981405689759e0_f64) * t2268 * t9178 + F::cast_from(0.34146007962811379518e0_f64) * t2268 * t9183 - F::cast_from(0.19918504644973304719e0_f64) * t2268 * t9186 - F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t9190 - F::cast_from(0.63233348079280332442e-2_f64) * t1358 * t9195 + F::cast_from(0.31616674039640166221e-2_f64) * t1358 * t9200;
    (t9199, t9203)
}
