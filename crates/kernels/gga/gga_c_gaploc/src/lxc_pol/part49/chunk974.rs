//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 974/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk974<F: Float>(t1063: F, t2765: F, t31300: F, t2268: F, t2343: F, t2787: F, t29853: F, t2854: F, t6320: F, t39764: F, t39766: F, t12770: F, t484: F) -> (F, F, F, F, F, F) {
    let t42763 = F::cast_from(0.85365019907028448797e-1_f64) * t1063 * t2765 * t31300;
    let t42767 = F::cast_from(0.34146007962811379518e0_f64) * t2268 * t2343 * t2787 * t29853;
    let t42771 = F::cast_from(0.17073003981405689759e0_f64) * t2268 * t6320 * t2854 * t29853;
    let t42772 = F::cast_from(0.31616674039640166221e-2_f64) * t39764;
    let t42773 = F::cast_from(0.31616674039640166221e-2_f64) * t39766;
    let t42774 = t484 * t12770;
    (t42763, t42767, t42771, t42772, t42773, t42774)
}
