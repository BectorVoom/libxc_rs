//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1096/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1096<F: Float>(t30268: F, t8783: F, t1479: F, t429: F, t1980: F, t7476: F, t1089: F, t15897: F, t2288: F, t598: F, t1988: F, t8486: F) -> (F, F, F, F, F) {
    let t35496 = t30268 * t8783;
    let t35500 = t429 * t1479;
    let t35502 = t1980 * t7476 * t35500;
    let t35511 = t598 * t1089 * t15897 * t2288;
    let t35513 = t1988 * t8486;
    (t35496, t35500, t35502, t35511, t35513)
}
