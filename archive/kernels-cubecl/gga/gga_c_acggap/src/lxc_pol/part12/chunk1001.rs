//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1001/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1001<F: Float>(t2294: F, t7630: F, t31253: F, t527: F, t2299: F, t7610: F, t2310: F, t7780: F, t137: F, t4838: F, t1083: F, t1089: F, t598: F) -> (F, F, F, F, F, F, F) {
    let t33865 = t7630 * t2294;
    let t33867 = t31253 * t527;
    let t33869 = t7610 * t2299;
    let t33874 = t7610 * t2310;
    let t33876 = t7780 * t2294;
    let t33878 = t137 * t4838;
    let t33881 = t598 * t1089 * t1083 * t33878;
    (t33865, t33867, t33869, t33874, t33876, t33878, t33881)
}
