//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1317/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1317<F: Float>(t2018: F, t807: F, t9703: F, t3994: F, t7028: F, t9845: F, t25240: F, t3951: F, t3964: F, t25972: F, t9761: F, t2681: F, t7269: F, t820: F) -> (F, F, F, F, F) {
    let t94534 = t807 * t2018 * t9703;
    let t94537 = t9845 * t7028 * t3994;
    let t94540 = t3964 * t25240 * t3951;
    let t94542 = t25972 * t9761;
    let t94545 = t820 * t7269 * t2681;
    (t94534, t94537, t94540, t94542, t94545)
}
