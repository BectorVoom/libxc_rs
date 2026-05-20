//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1675/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1675<F: Float>(t52128: F, t63453: F, t63459: F, t63464: F, t63533: F, t63538: F, t63545: F, t77559: F, t77561: F, t77806: F, t77858: F, t88252: F, t88257: F, t88260: F) -> F {
    let t88427 = F::cast_from(0.21908444444444444444e0_f64) * t77806 + F::cast_from(0.97370864197530864199e0_f64) * t52128 - F::cast_from(0.5314962962962962963e0_f64) * t63453 + F::cast_from(0.15944888888888888889e1_f64) * t63459 - F::cast_from(0.18257037037037037037e0_f64) * t63533 + F::cast_from(0.10954222222222222222e1_f64) * t63538 - F::cast_from(0.54771111111111111111e0_f64) * t63545 + F::cast_from(0.79724444444444444444e0_f64) * t77559 - F::cast_from(0.23917333333333333333e1_f64) * t77561 + F::new(0.3071625e0) * t88252 - F::cast_from(0.79724444444444444446e0_f64) * t63464 + F::cast_from(0.21908444444444444444e0_f64) * t77858 + F::cast_from(0.98587999999999999999e0_f64) * t88257 - F::new(0.295764e1) * t88260;
    t88427
}
