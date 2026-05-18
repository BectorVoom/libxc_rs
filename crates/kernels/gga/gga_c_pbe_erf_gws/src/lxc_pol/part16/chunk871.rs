//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 871/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk871<F: Float>(t610: F, t7468: F, t7467: F, t1820: F, t1033: F, t1683: F, t2816: F, t663: F, t2749: F, t633: F, t5338: F, t5347: F) -> (F, F, F, F, F, F) {
    let t7469 = t7468 * t610;
    let t7470 = t7467 * t7469;
    let t7472 = F::new(8.0) / F::new(15.0) * t1820 * t7470;
    let t7474 = F::new(8.0) / F::new(45.0) * t1033 * t1683;
    let t7476 = F::new(4.0) / F::new(15.0) * t2816 * t663;
    let t7478 = F::new(8.0) / F::new(45.0) * t633 * t2749;
    let t7479 = F::new(16.0) / F::new(45.0) * t5338;
    let t7480 = F::new(8.0) / F::new(45.0) * t5347;
    (t7472, t7474, t7476, t7478, t7479, t7480)
}
