//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1127/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1127<F: Float>(t33620: F, t644: F, t8621: F, t7714: F, t84: F, t640: F, t33624: F, t6972: F, t1497: F, t36: F, t606: F, t1936: F, t97622: F) -> (F, F, F, F, F) {
    let t125319 = t8621 * t33620 * t644;
    let t125322 = t84 * t7714;
    let t125324 = t8621 * t125322 * t640;
    let t125332 = t8621 * t33624 * t6972;
    let t125335 = t1497 * t36;
    let t125336 = t125335 * t606;
    let t125355 = t97622 * t1936;
    (t125319, t125324, t125332, t125336, t125355)
}
