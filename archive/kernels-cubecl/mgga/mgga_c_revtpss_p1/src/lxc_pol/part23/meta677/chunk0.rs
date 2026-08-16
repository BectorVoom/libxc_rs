//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2414/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2414<F: Float>(t13041: F, t44173: F, t13061: F, t13100: F, t828: F, t12879: F, t1247: F, t1251: F, t42994: F, t1231: F, t12898: F, t43813: F) -> (F, F, F, F, F, F, F) {
    let t44174 = t44173 * t13041;
    let t44202 = t44173 * t13061;
    let t44225 = t828 * t13100;
    let t44250 = t828 * t12879;
    let t44264 = t1247 * t42994 * t1251;
    let t44291 = t1231 * t12898;
    let t44307 = F::cast_from(0.86419753086419753087e-1_f64) * t43813;
    (t44174, t44202, t44225, t44250, t44264, t44291, t44307)
}
