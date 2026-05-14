//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 897/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk897<F: Float>(t10838: F, t2365: F, t8769: F, t6111: F, t3488: F, t826: F, t825: F, t10627: F, t701: F) -> (F, F, F, F, F, F) {
    let t10839 = 0.25561950635947166451e0 * t10838;
    let t10840 = t2365 * t8769;
    let t10841 = t6111 * t10840;
    let t10842 = 0.29792074959875355558e-1 * t10841;
    let t10843 = t826 * t3488;
    let t10844 = t825 * t10843;
    let t10845 = 0.25561950635947166451e0 * t10844;
    let t10847 = t10627 * t701;
    (t10839, t10840, t10842, t10843, t10845, t10847)
}
