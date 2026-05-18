//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 887/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk887<F: Float>(t39832: F, t8443: F, t41890: F, t39513: F, t8451: F, t10066: F, t36772: F, t40759: F, t8626: F, t623: F, t8629: F, t8632: F) -> (F, F, F, F, F, F) {
    let t44692 = t39832 * t8443;
    let t44694 = t41890 * t8443;
    let t44696 = t8451 * t39513;
    let t44700 = t36772 * t10066;
    let t44702 = t40759 * t8626;
    let t44705 = t623 * t8629 * t8632;
    (t44692, t44694, t44696, t44700, t44702, t44705)
}
