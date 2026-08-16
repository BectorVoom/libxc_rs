//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 148/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk148<F: Float>(t492: F, t493: F, t186: F, t137: F, t1: F, t124: F, t3: F, t4: F, t487: F, t141: F, t483: F, t486: F, t488: F) -> (F, F, F, F, F, F, F) {
    let t494 = t492 * t493;
    let t495 = F::cast_from(1.0_f64) / t186;
    let t496 = t137 * t495;
    let t498 = t124 * t1 * t3;
    let t499 = t496 * t498;
    let t502 = t487 * t4;
    let t505 = -F::cast_from(0.19415017735199121314e-1_f64) * t483 * t141 - F::cast_from(0.24268772168998901643e-2_f64) * t486 * t488 + F::cast_from(0.24268772168998901643e-3_f64) * t494 * t499 - F::cast_from(0.43149876916480047122e-3_f64) * t494 * t502;
    (t494, t495, t496, t498, t499, t502, t505)
}
