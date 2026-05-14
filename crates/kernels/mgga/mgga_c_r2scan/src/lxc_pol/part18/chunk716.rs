//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 716/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk716<F: Float>(t2049: F, t2312: F, t2317: F, t6804: F, t6806: F, t6809: F, t6868: F, t6874: F, t6876: F, t874: F, t875: F, t158: F, t166: F, t2320: F, t58: F, t766: F) -> (F, F, F) {
    let t6879 = 0.1714584e0 * t6804 - 0.1714584e0 * t6806 * t2049 + 0.285764e-1 * t6809 + 0.285764e-1 * t6868 * t875 - 0.857292e-1 * t2312 * t2317 * t874 + 0.571528e-1 * t6874 * t6876;
    let t6880 = t6879 * t158;
    let t6881 = t6880 * t166;
    let t6887 = t2320 * t58;
    let t6888 = t6887 * t766;
    (t6881, t6887, t6888)
}
