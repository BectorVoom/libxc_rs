//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1043/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1043<F: Float>(t150912: F, t27: F, t676: F, t89: F, t193: F, t33243: F, t3821: F, t1131: F, t140627: F, t35516: F, t713: F, t1434: F, t2506: F) -> (F, F, F, F, F) {
    let t150915 = t89 * t27 * t676 * t150912;
    let t150918 = t89 * t193 * t33243 * t3821;
    let t150922 = t89 * t193 * t140627 * t1131;
    let t150924 = t35516 * t713;
    let t150927 = t1434 * t193 * t2506 * t150924;
    (t150915, t150918, t150922, t150924, t150927)
}
