//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 522/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk522<F: Float>(t4533: F, t457: F, t91: F, t1832: F, t2981: F, t3006: F, t4420: F, t4424: F, t4428: F, t4434: F, t4439: F, t4498: F, t4507: F, t103: F, t82: F, t979: F) -> (F, F, F, F) {
    let t4535 = t91 * t457 * t4533;
    let t4545 = -t4507 / 12.0 + t4535 / 6.0 + t1832 + 2.0 / 27.0 * t2981 + 2.0 / 9.0 * t3006 - 2.0 / 27.0 * t4420 + 2.0 / 9.0 * t4424 + 2.0 / 9.0 * t4428 - t4434 / 9.0 + 2.0 / 3.0 * t4439 - t4498 / 3.0;
    let t4547 = t82 * t4545 * t103;
    let t4551 = t979 * t979;
    (t4535, t4545, t4547, t4551)
}
