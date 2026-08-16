//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1207/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1207<F: Float>(t19890: F, t5181: F, t3437: F, t19112: F, t388: F, t387: F, t1187: F, t1184: F, t6732: F, t19735: F, t3338: F, t3337: F) -> (F, F, F, F) {
    let t20169 = t5181 * t19890;
    let t20170 = t3437 * t20169;
    let t20172 = t388 * t19112;
    let t20173 = t387 * t20172;
    let t20174 = t1187 * t20173;
    let t20176 = t1184 * t6732;
    let t20178 = t3338 * t19735;
    let t20179 = t3337 * t20178;
    (t20170, t20174, t20176, t20179)
}
