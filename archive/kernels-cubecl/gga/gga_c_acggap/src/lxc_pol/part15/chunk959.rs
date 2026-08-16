//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 959/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk959<F: Float>(t7839: F, t8739: F, t1983: F, t30692: F, t5720: F, t7586: F, t8779: F, t1089: F, t535: F, t7553: F, t7554: F, t7637: F, t8491: F) -> (F, F, F, F, F) {
    let t33986 = t7839 * t8739;
    let t33994 = t30692 * t7586 * t1983 * t5720;
    let t33996 = t7839 * t8779;
    let t34009 = t7553 * t1089 * t535 * t7554;
    let t34011 = t7637 * t8491;
    (t33986, t33994, t33996, t34009, t34011)
}
