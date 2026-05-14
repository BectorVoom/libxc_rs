//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 872/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk872<F: Float>(t2310: F, t7944: F, t2191: F, t8597: F, t7939: F, t2283: F, t504: F, t8619: F, t8622: F, t38354: F, t7473: F, t7478: F, t35024: F, t8451: F, t36772: F, t8457: F) -> (F, F, F, F, F, F, F, F) {
    let t41863 = t7944 * t2310;
    let t41865 = t2191 * t8597;
    let t41882 = t7939 * t2310;
    let t41884 = t7939 * t2283;
    let t41886 = t504 * t8619;
    let t41887 = t41886 * t8622;
    let t41890 = t38354 * t7473;
    let t41891 = t41890 * t7478;
    let t41893 = t8451 * t35024;
    let t41895 = t36772 * t8457;
    (t41863, t41865, t41882, t41884, t41887, t41891, t41893, t41895)
}
