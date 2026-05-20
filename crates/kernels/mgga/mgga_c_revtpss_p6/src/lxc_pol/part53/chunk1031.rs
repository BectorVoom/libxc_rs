//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1031/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1031<F: Float>(t4254: F, t8457: F, t1936: F, t7221: F, t651: F, t7003: F, t8634: F, t196: F, t197: F, t7231: F, t2035: F, t6985: F) -> (F, F, F, F, F, F, F) {
    let t32309 = t4254 * t8457;
    let t32311 = t7221 * t1936;
    let t32312 = t651 * t32311;
    let t32320 = F::new(4.0) * t8634 * t7003;
    let t32322 = t7231 * t196 * t197;
    let t32323 = t32322 * t2035;
    let t32325 = t6985 * t7003;
    (t32309, t32311, t32312, t32320, t32322, t32323, t32325)
}
