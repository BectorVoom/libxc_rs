//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1828/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1828<F: Float>(t7063: F, t92889: F, t1955: F, t25308: F, t2769: F, t7036: F, t820: F, t844: F, t2751: F, t2482: F, t814: F, t10782: F) -> (F, F, F, F, F, F) {
    let t92890 = t7063 * t92889;
    let t92917 = t1955 * t25308 * t2769;
    let t92951 = t820 * t7036 * t844;
    let t92952 = t92951 * t2751;
    let t92955 = t2482 * t7036 * t814;
    let t92956 = t92955 * t10782;
    (t92890, t92917, t92951, t92952, t92955, t92956)
}
