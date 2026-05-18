//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 704/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk704<F: Float>(t10043: F, t201: F, t1979: F, t1982: F, t128: F, t1864: F, t118: F, t1986: F, t7408: F, t1737: F, t645: F, t4044: F) -> (F, F, F, F, F, F) {
    let t10044 = t10043 * t201;
    let t10046 = t10044 * t1979 * t1982;
    let t10047 = F::new(0.42564599893297839398e-5) * t10046;
    let t10048 = t128 * t1864;
    let t10049 = t118 * t10048;
    let t10050 = t1986 * t10049;
    let t10051 = t7408 * t10050;
    let t10052 = F::new(0.11971293719990017331e-4) * t10051;
    let t10053 = t645 * t1737;
    let t10054 = t4044 * t10053;
    (t10044, t10047, t10050, t10052, t10053, t10054)
}
