//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 715/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk715<F: Float>(t543: F, t9890: F, t1390: F, t828: F, t3926: F, t3930: F, t1398: F, t3923: F, t221: F, t4019: F, t4057: F, t4018: F, t1386: F, t2681: F, t820: F, t1401: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9891 = t9890 * t543;
    let t9893 = t1390 * t828 * t9891;
    let t9896 = t3930 * t3926;
    let t9898 = t3923 * t1398;
    let t9899 = t9898 * t543;
    let t9901 = t1390 * t828 * t9899;
    let t9905 = t4019 * t221 * t4057;
    let t9906 = t4018 * t9905;
    let t9909 = t820 * t1386 * t2681;
    let t9910 = t9909 * t1401;
    (t9891, t9893, t9896, t9898, t9899, t9901, t9905, t9906, t9910)
}
