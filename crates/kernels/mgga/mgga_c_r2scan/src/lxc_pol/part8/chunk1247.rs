//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1247/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1247<F: Float>(t277: F, t8691: F, t1610: F, t2201: F, t9434: F, t785: F, t788: F, t9365: F, t2837: F, t7476: F, t133: F, t255: F, t9083: F, t565: F, t25752: F, t7258: F) -> (F, F, F, F, F, F, F) {
    let t27914 = t277 * t8691;
    let t27934 = t2201 * t1610 * t9434;
    let t27938 = t2201 * t785 * t788 * t9365;
    let t27941 = t2201 * t2837 * t7476;
    let t27949 = t133 * t9083 * t255;
    let t27950 = t565 * t27949;
    let t27953 = t25752 * t7258;
    (t27914, t27934, t27938, t27941, t27949, t27950, t27953)
}
