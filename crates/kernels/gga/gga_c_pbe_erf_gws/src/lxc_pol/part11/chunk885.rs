//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 885/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk885<F: Float>(t2003: F, t245: F, t5926: F, t1999: F, t703: F, t1: F, t1478: F, t119: F, t671: F, t762: F, t39: F, t34: F, t413: F) -> (F, F, F, F, F, F, F) {
    let t16556 = F::cast_from(0.67090456446662028936e-1_f64) * t2003 * t245 * t5926;
    let t16561 = F::cast_from(0.44726970964441352624e-1_f64) * t2003 * t703 * t1999;
    let t16569 = t1478 * t1;
    let t16572 = F::cast_from(0.28503734567901234566e-4_f64) * t16569 * t119 * t671;
    let t16574 = F::cast_from(0.44134814814814814813e-2_f64) * t762 * t1999;
    let t16575 = F::new(72.0) * t39;
    let t16576 = t34 * t413;
    (t16556, t16561, t16569, t16572, t16574, t16575, t16576)
}
