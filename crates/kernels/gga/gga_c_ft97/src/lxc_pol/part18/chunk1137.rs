//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1137/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1137<F: Float>(t1882: F, t23492: F, t38953: F, t5857: F, t23994: F, t23999: F, t5882: F, t8232: F, t23436: F, t23524: F, t95051: F, t95054: F, t95078: F, t95087: F, t95094: F, t95107: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t95948 = t1882 * t23492;
    let t95954 = t38953 * t5857;
    let t95956 = t1882 * t23994;
    let t95958 = t1882 * t23999;
    let t95975 = t8232 * t5882;
    let t96002 = t1882 * t23436;
    let t96035 = t1882 * t23524;
    let t96050 = t95051 / 9.0;
    let t96051 = t95054 / 3.0;
    let t96057 = t95078 / 18.0;
    let t96060 = t95087 / 27.0;
    let t96062 = t95094 / 9.0;
    let t96066 = t95107 / 8.0;
    (t95948, t95954, t95956, t95958, t95975, t96002, t96035, t96050, t96051, t96057, t96060, t96062, t96066)
}
