//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 977/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk977<F: Float>(t3915: F, t5722: F, t1444: F, t1903: F, t4076: F, t1882: F, t555: F, t4086: F, t543: F, t2782: F, t1883: F, t72: F) -> (F, F, F, F, F, F) {
    let t5723 = t3915 * t5722;
    let t5727 = t1903 * t1444;
    let t5728 = t4076 * t5727;
    let t5735 = t555 * t1882;
    let t5737 = t4086 * t5735 * t543;
    let t5738 = t2782 * t5737;
    let t5740 = t1883 * t72;
    (t5723, t5728, t5735, t5737, t5738, t5740)
}
