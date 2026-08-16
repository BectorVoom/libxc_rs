//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 461/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk461<F: Float>(t1042: F, t1372: F, t1138: F, t1435: F, t5: F, t577: F, t946: F, t1009: F, t578: F, t1012: F, t1528: F, t195: F) -> (F, F, F, F, F, F) {
    let t5432 = t1372 * t1042;
    let t5434 = t1435 * t1138;
    let t5443 = t577 * t5;
    let t5444 = t5443 * t946;
    let t5446 = t1009 * t578;
    let t5448 = t1012 * t578;
    let t5452 = t195 * t1528;
    (t5432, t5434, t5444, t5446, t5448, t5452)
}
