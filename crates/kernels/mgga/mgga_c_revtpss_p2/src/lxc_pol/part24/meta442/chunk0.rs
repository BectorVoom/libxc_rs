//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1399/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1399<F: Float>(t220: F, t47273: F, t2482: F, t27: F, t9991: F, t1389: F, t3964: F, t40604: F, t39515: F, t4083: F, t14192: F, t555: F) -> (F, F, F, F, F) {
    let t47274 = t47273 * t220;
    let t47293 = t2482 * t9991 * t27;
    let t47337 = F::cast_from(0.11344944493805280483e-2_f64) * t3964 * t40604 * t1389;
    let t47351 = F::cast_from(0.11564373972601816912e-1_f64) * t39515 * t4083;
    let t47371 = t14192 * t555;
    (t47274, t47293, t47337, t47351, t47371)
}
