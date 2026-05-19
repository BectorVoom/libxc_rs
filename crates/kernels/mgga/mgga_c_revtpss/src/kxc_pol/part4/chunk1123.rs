//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1123/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1123<F: Float>(t1353: F, t5591: F, t4012: F, t828: F, t1868: F, t3889: F, t221: F, t5627: F, t9921: F, t3978: F, t13583: F, t13585: F, t13593: F, t13599: F, t13612: F, t13615: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F) -> (F, F, F, F) {
    let t13867 = t5591 * t1353;
    let t13869 = t4012 * t828 * t13867;
    let t13872 = t1868 * t3889;
    let t13874 = t4012 * t828 * t13872;
    let t13877 = t221 * t5627;
    let t13878 = t9921 * t13877;
    let t13880 = F::cast_from(0.50820002809285328225e-3_f64) * t3978 * t13878;
    let t13881 = t13583 + t13585 - t9278 + t9308 + t9316 + t9320 - t9325 + t9329 + t9333 - t9374 - t13593 - t9389 - t13599 - t9391 - t13612 - t13615;
    (t13869, t13874, t13880, t13881)
}
