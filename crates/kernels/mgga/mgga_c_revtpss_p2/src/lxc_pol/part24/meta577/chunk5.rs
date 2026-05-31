//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1774/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1774<F: Float>(t56236: F, t58153: F, t68399: F, t68583: F, t68585: F, t68590: F, t81236: F, t81491: F, t81496: F, t81539: F, t90486: F, t90488: F, t90490: F, t90492: F) -> F {
    let t90732 = -F::cast_from(0.68863333333333333332e0_f64) * t81236 - F::cast_from(0.21424148148148148148e1_f64) * t56236 + F::cast_from(0.27545333333333333333e1_f64) * t68399 - F::cast_from(0.166712e1_f64) * t81491 - F::cast_from(0.12349037037037037037e0_f64) * t81496 - F::cast_from(0.12349037037037037037e1_f64) * t58153 + F::cast_from(0.27785333333333333333e0_f64) * t81539 - F::cast_from(0.705945e1_f64) * t90486 + F::cast_from(0.158837625e2_f64) * t90488 - F::cast_from(0.94674375e0_f64) * t90490 + F::cast_from(0.1262325e1_f64) * t90492 + F::cast_from(0.69463333333333333334e0_f64) * t68583 + F::cast_from(0.13892666666666666667e1_f64) * t68585 - F::cast_from(0.23154444444444444445e0_f64) * t68590;
    t90732
}
