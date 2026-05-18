//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 839/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk839<F: Float>(t159: F, t619: F, t9767: F, t157: F, t1838: F, t609: F, t2152: F, t150: F, t187: F, t2331: F, t556: F, t2147: F) -> (F, F, F, F, F) {
    let t9769 = t619 * t159 * t9767;
    let t9773 = t609 * t1838 * t157;
    let t9774 = t2152 * t9773;
    let t9779 = t9767 * t150 * t187;
    let t9789 = t2331 * t556;
    let t9790 = t2147 * t9789;
    (t9769, t9774, t9779, t9789, t9790)
}
