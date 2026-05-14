//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 864/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk864<F: Float>(t1882: F, t34187: F, t34246: F, t34217: F, t34126: F, t7674: F, t8232: F, t34221: F, t681: F, t89: F, t34242: F, t34213: F, t7635: F, t7681: F, t34236: F, t2399: F, t7664: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t144153 = t1882 * t34187;
    let t144162 = t1882 * t34246;
    let t144176 = t1882 * t34217;
    let t144178 = t1882 * t34126;
    let t144184 = 8.0 / 27.0 * t8232 * t7674;
    let t144190 = t89 * t681 * t34221;
    let t144197 = t1882 * t34242;
    let t144199 = t1882 * t34213;
    let t144212 = 4.0 / 27.0 * t8232 * t7635;
    let t144219 = 4.0 / 27.0 * t8232 * t7681;
    let t144227 = t1882 * t34236;
    let t144236 = 4.0 / 27.0 * t89 * t2399 * t7664;
    (t144153, t144162, t144176, t144178, t144184, t144190, t144197, t144199, t144212, t144219, t144227, t144236)
}
