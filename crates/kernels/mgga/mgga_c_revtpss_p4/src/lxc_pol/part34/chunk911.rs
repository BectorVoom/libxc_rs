//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 911/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk911<F: Float>(t5892: F, t625: F, t5916: F, t1450: F, t6922: F, t1882: F, t1892: F, t555: F, t6861: F, t6843: F, t550: F, t543: F) -> (F, F, F, F, F, F, F) {
    let t21818 = t625 * t5892;
    let t21827 = t625 * t5916;
    let t21937 = t6922 * t1450;
    let t21981 = t1892 * t1882;
    let t22005 = t555 * t6861;
    let t22009 = t555 * t6843;
    let t22020 = t550 * t6843;
    let t22021 = t22020 * t543;
    (t21818, t21827, t21937, t21981, t22005, t22009, t22021)
}
