//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1299/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1299<F: Float>(t2014: F, t22475: F, t7934: F, t29996: F, t7898: F, t30005: F, t7732: F, t30128: F, t1936: F, t25043: F, t651: F, t28172: F, t29494: F) -> (F, F, F, F, F, F) {
    let t113076 = F::new(6.0) * t2014 * t7934 * t22475;
    let t113078 = F::new(6.0) * t7898 * t29996;
    let t113084 = F::new(6.0) * t7732 * t30005;
    let t113086 = F::new(6.0) * t7732 * t30128;
    let t113089 = F::new(2.0) * t651 * t25043 * t1936;
    let t113092 = F::new(9.0) * t2014 * t28172 * t29494;
    (t113076, t113078, t113084, t113086, t113089, t113092)
}
