//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 872/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk872<F: Float>(t11763: F, t2028: F, t2536: F, t787: F, t13506: F, t4673: F, t6060: F, t2087: F, t4614: F, t43715: F, t10931: F, t23220: F, t45316: F) -> (F, F, F, F, F) {
    let t45557 = F::cast_from(0.39722766613167140743e-1_f64) * t787 * t2536 * t11763 * t2028;
    let t45560 = F::cast_from(0.14300195980740170667e1_f64) * t6060 * t4673 * t13506;
    let t45563 = F::cast_from(0.82820720060468819301e2_f64) * t2087 * t4614 * t13506;
    let t45565 = F::cast_from(0.23833659967900284446e0_f64) * t43715;
    let t45569 = F::cast_from(0.27606906686822939767e2_f64) * t23220 * t10931 * t45316;
    (t45557, t45560, t45563, t45565, t45569)
}
