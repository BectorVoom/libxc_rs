//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1464/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1464<F: Float>(t41306: F, t41292: F, t41299: F, t41303: F, t41341: F, t41344: F, t41347: F, t41350: F, t41361: F, t41363: F, t41369: F, t41373: F, t41384: F, t41387: F) -> F {
    let t41690 = F::cast_from(0.5356037037037037037e1_f64) * t41306;
    let t41701 = F::cast_from(0.12349037037037037037e0_f64) * t41292 - F::cast_from(0.10805407407407407407e0_f64) * t41299 - F::new(0.104195e0) * t41303 + t41690 + F::new(0.6311625e0) * t41373 - F::cast_from(0.15302962962962962963e1_f64) * t41341 - F::new(0.516475e0) * t41344 - F::new(0.123954e2) * t41347 + F::cast_from(0.68863333333333333334e1_f64) * t41350 + F::cast_from(0.21424148148148148148e1_f64) * t41361 + F::cast_from(0.27545333333333333333e1_f64) * t41363 - F::cast_from(0.27545333333333333332e1_f64) * t41369 + F::cast_from(0.2366859375e0_f64) * t41384 + F::new(0.94674375e0) * t41387;
    t41701
}
