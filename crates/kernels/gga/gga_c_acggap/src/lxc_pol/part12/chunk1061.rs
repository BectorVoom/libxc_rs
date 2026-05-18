//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1061/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1061<F: Float>(t2030: F, t301: F, t4262: F, t8484: F, t2060: F, t372: F, t8927: F, t1072: F, t535: F, t7507: F, t7512: F, t1131: F, t2288: F) -> (F, F, F, F) {
    let t34869 = t2030 * t4262 * t8484 * t301;
    let t34873 = t2060 * t8927 * t8484 * t372;
    let t34879 = t7507 * t7512 * t535 * t1072;
    let t34883 = t2060 * t8927 * t2288 * t1131;
    (t34869, t34873, t34879, t34883)
}
