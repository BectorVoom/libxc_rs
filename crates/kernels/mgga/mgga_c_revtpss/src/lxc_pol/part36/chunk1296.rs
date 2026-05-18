//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1296/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1296<F: Float>(t2142: F, t6564: F, t30840: F, t460: F, t1769: F, t1828: F, t1032: F, t6695: F, t2148: F, t1209: F, t30882: F, t7658: F) -> (F, F, F, F, F, F, F) {
    let t112706 = t6564 * t2142;
    let t112714 = t460 * t30840;
    let t112721 = t1769 * t1828;
    let t112757 = t6695 * t1032;
    let t112758 = t2148 * t112757;
    let t112774 = t1209 * t112757;
    let t112843 = t30882 * t7658;
    (t112706, t112714, t112721, t112757, t112758, t112774, t112843)
}
