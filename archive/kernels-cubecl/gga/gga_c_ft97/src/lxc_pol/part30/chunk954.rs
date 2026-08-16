//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 954/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk954<F: Float>(t7555: F, t8232: F, t1882: F, t33617: F, t33638: F, t737: F, t7536: F, t33605: F, t7508: F, t7504: F, t33665: F, t33673: F) -> (F, F, F, F, F, F, F, F, F) {
    let t142002 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t8232 * t7555;
    let t142009 = t1882 * t33617;
    let t142020 = t1882 * t33638;
    let t142030 = t737 * t7536;
    let t142058 = t1882 * t33605;
    let t142074 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t8232 * t7508;
    let t142083 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t8232 * t7504;
    let t142117 = t1882 * t33665;
    let t142135 = t1882 * t33673;
    (t142002, t142009, t142020, t142030, t142058, t142074, t142083, t142117, t142135)
}
