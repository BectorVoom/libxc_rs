//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2455/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2455<F: Float>(t1389: F, t3964: F, t40604: F, t10111: F, t22: F, t4092: F, t39515: F, t4083: F, t10043: F, t9303: F, t14192: F, t555: F) -> (F, F, F, F, F) {
    let t47337 = F::cast_from(0.11344944493805280483e-2_f64) * t3964 * t40604 * t1389;
    let t47348 = t10111 * t4092 * t22;
    let t47351 = F::cast_from(0.11564373972601816912e-1_f64) * t39515 * t4083;
    let t47352 = t9303 * t10043;
    let t47371 = t14192 * t555;
    (t47337, t47348, t47351, t47352, t47371)
}
