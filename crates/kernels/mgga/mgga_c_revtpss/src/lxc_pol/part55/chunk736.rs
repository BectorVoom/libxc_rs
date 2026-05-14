//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 736/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk736<F: Float>(t118: F, t8686: F, t1936: F, t7359: F, t93: F, t2055: F, t196: F, t2093: F, t197: F) -> (F, F, F, F, F, F) {
    let t8687 = t118 * t8686;
    let t8689 = 2.0 * t7359 * t1936;
    let t8692 = t93 * t1936;
    let t8694 = 2.0 * t8692 * t2055;
    let t8697 = t2093 * t196;
    let t8698 = t8697 * t197;
    (t8687, t8689, t8692, t8694, t8697, t8698)
}
