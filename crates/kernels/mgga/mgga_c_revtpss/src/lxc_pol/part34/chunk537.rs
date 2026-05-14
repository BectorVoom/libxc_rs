//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 537/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk537<F: Float>(t1892: F, t213: F, t1357: F, t1904: F, t689: F, t1903: F, t72: F, t686: F, t3915: F, t1882: F, t555: F, t4086: F, t543: F, t2782: F, t1883: F, t4101: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5715 = t213 * t1892;
    let t5718 = t1357 * t1904;
    let t5719 = t689 * t5718;
    let t5721 = t1903 * t72;
    let t5722 = t5721 * t686;
    let t5723 = t3915 * t5722;
    let t5735 = t555 * t1882;
    let t5737 = t4086 * t5735 * t543;
    let t5738 = t2782 * t5737;
    let t5740 = t1883 * t72;
    let t5741 = t5740 * t686;
    let t5742 = t4101 * t5741;
    (t5715, t5718, t5719, t5721, t5722, t5723, t5735, t5737, t5738, t5740, t5741, t5742)
}
