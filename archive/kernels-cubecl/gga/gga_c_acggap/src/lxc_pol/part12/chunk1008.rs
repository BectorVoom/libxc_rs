//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1008/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1008<F: Float>(t1181: F, t22778: F, t7564: F, t8600: F, t1983: F, t30692: F, t5720: F, t7586: F, t7839: F, t8779: F, t4991: F, t7822: F) -> (F, F, F, F) {
    let t33990 = t7564 * t1181 * t8600 * t22778;
    let t33994 = t30692 * t7586 * t1983 * t5720;
    let t33996 = t7839 * t8779;
    let t33998 = t7822 * t4991;
    (t33990, t33994, t33996, t33998)
}
