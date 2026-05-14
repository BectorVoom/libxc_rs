//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 327/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk327<F: Float>(t1592: F, t904: F, t128: F, t903: F, t291: F, t902: F, t916: F, t923: F, t930: F, t141: F, t921: F, t929: F, t935: F, t915: F, t939: F, t948: F, t951: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1593 = t904 * t1592;
    let t1594 = t128 * t1593;
    let t1596 = -t903 - 0.17808333333333333333e-1 * t1594;
    let t1598 = 0.621814e-1 * t1596 * t291;
    let t1600 = -t902 / 3.0 - t1594 / 3.0;
    let t1601 = t916 * t1600;
    let t1604 = t923 * t1600;
    let t1606 = t930 * t1592;
    let t1607 = t141 * t1606;
    let t1609 = 0.1898925e1 * t1601 - t921 - 0.29896666666666666667e0 * t1594 + 0.3071625e0 * t1604 - t929 - 0.82156666666666666667e-1 * t1607;
    let t1610 = t1609 * t935;
    let t1612 = 1.0 * t915 * t1610;
    let t1614 = -t939 - 0.17123333333333333333e-1 * t1594;
    let t1621 = 0.3529725e1 * t1601 - t948 - 0.516475e0 * t1594 + 0.6311625e0 * t1604 - t951 - 0.104195e0 * t1607;
    (t1593, t1594, t1596, t1598, t1600, t1601, t1604, t1606, t1607, t1609, t1610, t1612, t1614, t1621)
}
