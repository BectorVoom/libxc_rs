//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1388/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1388<F: Float>(t18930: F, t18934: F, t18941: F, t18973: F, t18979: F, t18984: F, t18990: F, t22616: F, t23719: F, t32125: F, t32127: F, t32133: F, t18995: F, t19013: F, t23741: F, t23752: F, t26918: F, t26922: F, t26924: F, t26928: F, t32108: F, t32134: F, t32139: F, t765: F) -> (F, F) {
    let t33733 = t18930 - t32125 + t32127 - t18934 + t18941 - t23719 - t18973 + t18979 - t18984 + t32133 + t18990 + 0.1714584e0 * t22616;
    let t33739 = t23741 + t32134 - t18995 + t26918 + t26922 - 0.12154685976e1 * t26924 - t26928 - t32139 - t23752 - t19013 + 0.2025780996e0 * t765 * t32108;
    (t33733, t33739)
}
