//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 748/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk748<F: Float>(t1936: F, t6985: F, t8453: F, t93: F, t1312: F, t8460: F, t8447: F, t196: F, t2011: F, t197: F) -> (F, F, F, F) {
    let t8559 = t6985 * t1936;
    let t8562 = F::new(2.0) * t93 * t8453;
    let t8563 = t1312 * t8460;
    let t8564 = F::new(2.0) * t8563;
    let t8565 = t8447 + F::new(4.0) * t8559 + t8562 + t8564;
    let t8567 = t2011 * t196;
    let t8568 = t8567 * t197;
    (t8564, t8565, t8567, t8568)
}
