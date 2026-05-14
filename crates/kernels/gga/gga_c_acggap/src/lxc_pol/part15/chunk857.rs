//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 857/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk857<F: Float>(t2241: F, t30005: F, t2217: F, t310: F, t1603: F, t618: F, t2137: F, t525: F, t879: F, t545: F, t7923: F, t1411: F, t309: F, t615: F, t8396: F, t862: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33321 = t30005 * t2241;
    let t33323 = t310 * t2217;
    let t33428 = t1603 * t618;
    let t33429 = t2137 * t33428;
    let t33509 = t525 * t879;
    let t33524 = t7923 * t545;
    let t33551 = t1411 * t309;
    let t33566 = t615 * t33428;
    let t33574 = t862 * t8396;
    (t33321, t33323, t33428, t33429, t33509, t33524, t33551, t33566, t33574)
}
