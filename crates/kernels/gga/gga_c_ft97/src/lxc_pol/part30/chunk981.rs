//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 981/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk981<F: Float>(t34083: F, t8392: F, t1882: F, t34095: F, t34178: F, t34183: F, t34187: F, t34246: F, t34217: F, t34126: F, t7674: F, t8232: F) -> (F, F, F, F, F, F, F, F, F) {
    let t144140 = t8392 * t34083;
    let t144142 = t1882 * t34095;
    let t144148 = t1882 * t34178;
    let t144150 = t1882 * t34183;
    let t144153 = t1882 * t34187;
    let t144162 = t1882 * t34246;
    let t144176 = t1882 * t34217;
    let t144178 = t1882 * t34126;
    let t144184 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t8232 * t7674;
    (t144140, t144142, t144148, t144150, t144153, t144162, t144176, t144178, t144184)
}
