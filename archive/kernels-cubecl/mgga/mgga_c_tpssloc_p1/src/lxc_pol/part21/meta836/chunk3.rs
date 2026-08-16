//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2973/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2973<F: Float>(t10422: F, t17704: F, t3070: F, t17680: F, t1041: F, t13969: F, t17692: F, t10408: F, t10413: F, t10937: F, t17697: F, t17705: F, t17984: F, t2771: F, t3048: F, t42334: F, t42388: F, t42586: F, t4575: F, t4600: F, t48477: F, t48607: F, t48611: F, t48612: F, t49690: F, t49692: F, t49697: F, t49771: F, t49984: F, t5878: F, t61098: F) -> F {
    let t62013 = t3070 * t10422 * t17704;
    let t62032 = t3070 * t10422 * t17680;
    let t62038 = t1041 * t13969 * t17692;
    let t62042 = -t49771 * t4600 / F::cast_from(768.0_f64) - t10937 * t17705 / F::cast_from(216.0_f64) + t62013 / F::cast_from(1728.0_f64) - t42586 / F::cast_from(6912.0_f64) - t49690 / F::cast_from(3456.0_f64) - t49692 / F::cast_from(5184.0_f64) - t42334 * t17984 / F::cast_from(256.0_f64) - F::cast_from(5.0_f64) / F::cast_from(576.0_f64) * t48607 * t10408 * t61098 - F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t10413 * t10408 * t5878 * t2771 + t42388 * t48611 * t48612 * t48477 / F::cast_from(128.0_f64) + t62032 / F::cast_from(3456.0_f64) - t49984 * t4575 / F::cast_from(216.0_f64) + t49697 / F::cast_from(1728.0_f64) + F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t62038 - F::cast_from(5.0_f64) / F::cast_from(486.0_f64) * t3048 * t17697;
    t62042
}
