//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 496/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk496<F: Float>(t9308: F, t9349: F, t9388: F, t9434: F, t9475: F, t9509: F, t9551: F, t9585: F, t2530: F, t2581: F, t2580: F, t3234: F, t325: F) -> (F, F, F, F) {
    let t9588 = t9308 + t9349 + t9388 + t9434 + t9475 + t9509 + t9551 + t9585;
    let t9591 = t2581 * t2530;
    let t9592 = t2580 * t9591;
    let t9595 = t325 * t3234;
    (t9588, t9591, t9592, t9595)
}
