//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1104/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1104<F: Float>(t2722: F, t886: F, t2723: F, t1032: F, t2760: F, t867: F, t7063: F, t7060: F, t136: F, t2457: F, t7082: F, t25299: F, t212: F, t25286: F, t689: F, t780: F) -> (F, F, F, F, F, F, F, F) {
    let t92883 = t886 * t2722;
    let t92884 = t92883 * t2723;
    let t92888 = t2760 * t1032;
    let t92889 = t92888 * t867;
    let t92890 = t7063 * t92889;
    let t92891 = t92890 * t7060;
    let t92894 = t7082 * t136 * t2457;
    let t92895 = t25299 * t92894;
    let t92901 = t689 * t212 * t25286 * t780;
    (t92883, t92884, t92888, t92889, t92891, t92894, t92895, t92901)
}
