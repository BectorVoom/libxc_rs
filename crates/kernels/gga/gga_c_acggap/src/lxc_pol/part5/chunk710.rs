//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 710/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk710<F: Float>(t175: F, t398: F, t5080: F, t1413: F, t935: F, t506: F, t879: F, t368: F, t384: F, t3476: F, t527: F, t1017: F) -> (F, F, F, F, F, F, F) {
    let t5082 = t398 * t175 * t5080;
    let t5086 = F::new(0.42874018118069736972e-3) * t935 * t1413;
    let t5087 = t506 * t879;
    let t5089 = t398 * t368 * t5087;
    let t5090 = t384 * t5089;
    let t5092 = t3476 * t527;
    let t5094 = t506 * t1017;
    (t5082, t5086, t5087, t5089, t5090, t5092, t5094)
}
