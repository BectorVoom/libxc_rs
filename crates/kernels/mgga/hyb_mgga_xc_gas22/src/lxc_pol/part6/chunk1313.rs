//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1313/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1313<F: Float>(t7: F, t132: F, t27905: F, t27935: F, t27990: F, t28043: F, t28084: F, t28128: F, t28645: F, t28693: F, t24480: F, t10658: F, t20895: F, t2189: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t28697 = piecewise3::<F>(t134, F::new(0.0), t27905 + t27935 + t27990 + t28043 + t28084 + t28128 + t28645 + t28693);
    let t28698 = piecewise3::<F>(t8, F::new(0.0), t24480);
    let t28730 = F::cast_from(0.62071215503128080361e4_f64) * t20895 * t10658 * t2189;
    (t28697, t28698, t28730)
}
