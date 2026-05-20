//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1643/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1643<F: Float>(t20823: F, t5268: F, t1042: F, t5265: F, t5274: F, t1774: F, t3362: F, t4181: F, t12787: F, t12916: F, t6689: F, t3718: F) -> (F, F, F, F, F) {
    let t20913 = t5268 * t20823;
    let t20914 = t1042 * t20913;
    let t20917 = t5274 * t5265;
    let t20921 = t1774 * t3362;
    let t20922 = t20921 * t4181;
    let t20923 = t12787 * t20922;
    let t20926 = t12916 * t6689;
    let t20927 = t3718 * t20926;
    (t20914, t20917, t20923, t20926, t20927)
}
