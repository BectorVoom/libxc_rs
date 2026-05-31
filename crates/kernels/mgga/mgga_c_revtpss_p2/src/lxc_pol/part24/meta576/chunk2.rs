//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1764/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1764<F: Float>(t56236: F, t58153: F, t68399: F, t68583: F, t68585: F, t68590: F, t81236: F, t81491: F, t81496: F, t81539: F, t90486: F, t90488: F, t90490: F, t90492: F) -> F {
    let t90573 = -F::cast_from(0.39862222222222222223e0_f64) * t81236 - F::cast_from(0.12401580246913580247e1_f64) * t56236 + F::cast_from(0.15944888888888888889e1_f64) * t68399 - F::cast_from(0.13145066666666666666e1_f64) * t81491 - F::cast_from(0.97370864197530864196e-1_f64) * t81496 - F::cast_from(0.97370864197530864199e0_f64) * t58153 + F::cast_from(0.21908444444444444444e0_f64) * t81539 - F::cast_from(0.379785e1_f64) * t90486 + F::cast_from(0.85451625e1_f64) * t90488 - F::cast_from(0.46074375e0_f64) * t90490 + F::cast_from(0.614325e0_f64) * t90492 + F::cast_from(0.54771111111111111111e0_f64) * t68583 + F::cast_from(0.10954222222222222222e1_f64) * t68585 - F::cast_from(0.18257037037037037037e0_f64) * t68590;
    t90573
}
