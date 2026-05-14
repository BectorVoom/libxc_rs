//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 599/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk599<F: Float>(t7474: F, t8443: F, t1502: F, t236: F, t1971: F, t1970: F, t2313: F, t5542: F) -> (F, F, F, F) {
    let t8444 = t7474 * t8443;
    let t8446 = t236 * t1502;
    let t8447 = t1971 * t8446;
    let t8448 = t1970 * t8447;
    let t8450 = t2313 * t5542;
    (t8444, t8447, t8448, t8450)
}
