//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 283/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk283<F: Float>(t920: F, t924: F, t935: F, t1036: F, t245: F, t934: F) -> (F, F, F) {
    let t1040 = 0.41275e-2 * t920;
    let t1042 = 0.1982e-1 * t935 - t1040 - 0.41275e-2 * t924;
    let t1045 = t1036 * t934 / 4.0 + t245 * t1042 / 2.0;
    (t1040, t1042, t1045)
}
