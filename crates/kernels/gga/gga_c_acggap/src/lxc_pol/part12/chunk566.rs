//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 566/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk566<F: Float>(t2843: F, t2845: F, t2847: F, t1388: F, t224: F, t1: F, t1378: F, t283: F, t2894: F, t1390: F, t229: F, t276: F) -> (F, F, F, F, F, F, F, F) {
    let t4042 = F::new(4.0) * t2843;
    let t4043 = F::new(4.0) * t2845;
    let t4044 = F::new(32.0) * t2847;
    let t4045 = t224 * t1388;
    let t4046 = F::new(8.0) * t4045;
    let t4047 = t1378 * t1;
    let t4048 = t4047 * t283;
    let t4049 = F::new(0.36622894612013090108e-3) * t4048;
    let t4050 = F::new(12.0) * t2894;
    let t4057 = t229 * t1390;
    let t4058 = F::new(8.0) * t4057;
    let t4059 = t1378 * t276;
    (t4042, t4043, t4044, t4046, t4049, t4050, t4058, t4059)
}
