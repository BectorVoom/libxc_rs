//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 330/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk330<F: Float>(t1600: F, t916: F, t923: F, t1592: F, t930: F, t141: F, t1594: F, t921: F, t929: F, t935: F, t915: F, t939: F, t948: F, t951: F, t954: F, t958: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1601 = t916 * t1600;
    let t1604 = t923 * t1600;
    let t1606 = t930 * t1592;
    let t1607 = t141 * t1606;
    let t1609 = 0.1898925e1 * t1601 - t921 - 0.29896666666666666667e0 * t1594 + 0.3071625e0 * t1604 - t929 - 0.82156666666666666667e-1 * t1607;
    let t1610 = t1609 * t935;
    let t1612 = 1.0 * t915 * t1610;
    let t1614 = -t939 - 0.17123333333333333333e-1 * t1594;
    let t1621 = 0.3529725e1 * t1601 - t948 - 0.516475e0 * t1594 + 0.6311625e0 * t1604 - t951 - 0.104195e0 * t1607;
    let t1622 = t1621 * t954;
    let t1626 = -t958 - 0.92708333333333333333e-2 * t1594;
    (t1601, t1604, t1606, t1607, t1609, t1610, t1612, t1614, t1621, t1622, t1626)
}
