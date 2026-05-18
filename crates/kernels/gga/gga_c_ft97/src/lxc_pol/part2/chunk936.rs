//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 936/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk936<F: Float>(t10304: F, t1095: F, t13540: F, t13571: F, t801: F, t13542: F, t10883: F, t13538: F, t13547: F, t13553: F, t13556: F, t13562: F, t13565: F, t2380: F) -> (F, F, F) {
    let t14541 = t10304 * t1095;
    let t14544 = F::new(0.6419148148148148148e-1) * t13540;
    let t14550 = t801 * t13571;
    let t14553 = F::new(0.19257444444444444444e0) * t13542;
    let t14554 = F::new(0.1760655e0) * t14541 * t2380 + t14544 - F::new(0.9628722222222222222e-1) * t13556 - F::new(0.1604787037037037037e0) * t13547 + F::new(0.38514888888888888888e0) * t13553 + F::new(0.28886166666666666666e0) * t13565 - F::new(0.11554466666666666666e1) * t13562 + F::new(0.234754e0) * t14550 - t10883 - F::new(0.6419148148148148148e-1) * t13538 - t14553;
    (t14541, t14550, t14554)
}
