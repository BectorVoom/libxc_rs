//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 961/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk961<F: Float>(t1518: F, t6982: F, t1936: F, t1931: F, t4292: F, t33602: F, t7002: F, t25805: F, t7741: F, t28025: F, t28042: F, t6985: F, t34258: F, t32392: F, t32655: F, t8692: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t125362 = t6982 * t1518;
    let t125363 = t125362 * t1936;
    let t125365 = t1931 * t4292;
    let t125366 = t125365 * t1936;
    let t125368 = t33602 * t7002;
    let t125370 = t25805 * t7741;
    let t125372 = t28025 * t7741;
    let t125374 = t6985 * t28042;
    let t125377 = 4.0 * t34258 * t7002;
    let t125379 = 4.0 * t32392 * t7741;
    let t125381 = 4.0 * t32655 * t7741;
    let t125383 = 4.0 * t8692 * t28042;
    (t125362, t125363, t125365, t125366, t125368, t125370, t125372, t125374, t125377, t125379, t125381, t125383)
}
