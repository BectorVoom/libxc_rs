//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 825/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk825<F: Float>(t10631: F, t808: F, t10886: F, t2699: F, t798: F, t802: F, t2703: F, t2707: F, t10489: F, t124: F, t800: F, t159: F, t853: F, t216: F, t10627: F, t2729: F, t794: F) -> (F, F, F, F, F, F, F) {
    let t10887 = t808 * t10631;
    let t10888 = t10886 * t10887;
    let t10890 = t2699 * t798;
    let t10891 = t10890 * t802;
    let t10893 = t2703 * t2707;
    let t10895 = t124 * t10489;
    let t10896 = t800 * t10895;
    let t10899 = t159 * t853;
    let t10900 = t216 * t10899;
    let t10902 = t800 * t124 * t10627;
    let t10905 = t794 * t2729;
    (t10888, t10891, t10893, t10896, t10900, t10902, t10905)
}
