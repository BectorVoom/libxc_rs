//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 719/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk719<F: Float>(t2372: F, t255: F, t1131: F, t761: F, t1882: F, t3999: F, t3995: F, t1160: F, t2567: F, t3894: F, t8392: F, t3877: F) -> (F, F, F, F, F, F, F) {
    let t13885 = t2372 * t255;
    let t13886 = t761 * t1131;
    let t13903 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3999;
    let t13905 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3995;
    let t13927 = t1160 * t2567;
    let t13933 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t8392 * t3894;
    let t13959 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8392 * t3877;
    (t13885, t13886, t13903, t13905, t13927, t13933, t13959)
}
