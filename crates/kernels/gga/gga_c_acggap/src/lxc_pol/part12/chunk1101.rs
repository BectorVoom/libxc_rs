//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1101/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1101<F: Float>(t7433: F, t8481: F, t4680: F, t8463: F, t8652: F, t34161: F, t8465: F, t1992: F, t7585: F, t7842: F, t8402: F, t8787: F) -> (F, F, F, F, F) {
    let t35596 = t7433 * t8481;
    let t35599 = t8463 * t4680 * t8652;
    let t35601 = t34161 * t8465;
    let t35608 = t7585 * t7842 * t1992 * t8402;
    let t35610 = t7433 * t8787;
    (t35596, t35599, t35601, t35608, t35610)
}
