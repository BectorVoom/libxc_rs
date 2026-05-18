//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1123/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1123<F: Float>(t1850: F, t9633: F, t9760: F, t7173: F, t9647: F, t9648: F, t29439: F, t9652: F, t2554: F, t7064: F, t7280: F, t21665: F) -> (F, F, F, F, F, F) {
    let t29455 = F::new(0.17090058289204942853e-2) * t1850 * t9633;
    let t29457 = F::new(0.17090058289204942853e-2) * t1850 * t9760;
    let t29471 = F::new(0.1922631557535556071e-2) * t9647 * t9648 * t7173;
    let t29473 = F::new(0.2563508743380741428e-2) * t29439 * t9652;
    let t29476 = F::new(0.1281754371690370714e-2) * t7064 * t7280 * t2554;
    let t29478 = F::new(0.1281754371690370714e-2) * t21665 * t9633;
    (t29455, t29457, t29471, t29473, t29476, t29478)
}
