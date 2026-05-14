//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1403/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1403<F: Float>(t117310: F, t7317: F, t117349: F, t9977: F, t4817: F, t8972: F, t117400: F, t9972: F, t2454: F, t2559: F, t9709: F, t1873: F, t24487: F, t6974: F, t7327: F, t6719: F, t7413: F) -> (F, F, F, F, F, F, F, F) {
    let t122239 = t117310 * t7317;
    let t122241 = t117349 * t9977;
    let t122243 = t4817 * t8972;
    let t122245 = t117400 * t9972;
    let t122247 = t2559 * t2454;
    let t122248 = t122247 * t9709;
    let t122250 = t1873 * t24487;
    let t122252 = t6974 * t7327;
    let t122254 = t6719 * t7413;
    (t122239, t122241, t122243, t122245, t122248, t122250, t122252, t122254)
}
