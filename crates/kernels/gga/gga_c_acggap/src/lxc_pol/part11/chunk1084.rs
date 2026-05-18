//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1084/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1084<F: Float>(t1072: F, t535: F, t7507: F, t7512: F, t1131: F, t2060: F, t2288: F, t8927: F, t2297: F, t4256: F, t7450: F, t839: F) -> (F, F, F) {
    let t34879 = t7507 * t7512 * t535 * t1072;
    let t34883 = t2060 * t8927 * t2288 * t1131;
    let t34887 = t7450 * t4256 * t2297 * t839;
    (t34879, t34883, t34887)
}
