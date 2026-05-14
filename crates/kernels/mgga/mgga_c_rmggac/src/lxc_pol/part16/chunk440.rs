//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 440/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk440<F: Float>(t433: F, t5400: F, t1415: F, t385: F, t1413: F, t381: F, t1131: F, t577: F, t155: F, t1042: F, t1372: F, t1138: F, t1435: F, t5: F, t946: F, t1009: F, t578: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5402 = 0.11696447245269292414e1 * t5400 * t433;
    let t5404 = t385 * t1415;
    let t5407 = 8.0 * t381 * t1413;
    let t5409 = 8.0 * t385 * t1413;
    let t5419 = t577 * t1131;
    let t5420 = t155 * t5419;
    let t5432 = t1372 * t1042;
    let t5434 = t1435 * t1138;
    let t5443 = t577 * t5;
    let t5444 = t5443 * t946;
    let t5446 = t1009 * t578;
    (t5402, t5404, t5407, t5409, t5420, t5432, t5434, t5444, t5446)
}
