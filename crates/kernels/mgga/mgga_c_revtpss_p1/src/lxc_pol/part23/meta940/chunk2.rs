//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3089/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3089<F: Float>(t24297: F, t698: F, t58225: F, t68454: F, t68456: F, t68538: F, t68540: F, t68548: F, t68550: F, t68567: F, t68583: F, t68585: F, t68590: F) -> (F, F) {
    let t81539 = t698 * t24297;
    let t81552 = F::cast_from(0.54771111111111111112e-1_f64) * t81539 - F::cast_from(0.65725333333333333332e0_f64) * t68538 - F::cast_from(0.98587999999999999998e0_f64) * t68540 + F::cast_from(0.10954222222222222222e0_f64) * t68548 + F::cast_from(0.32862666666666666666e0_f64) * t68550 - F::cast_from(0.11958666666666666667e1_f64) * t68454 - F::cast_from(0.17938e1_f64) * t68456 - F::cast_from(0.16431333333333333333e0_f64) * t68567 + F::cast_from(0.54771111111111111112e0_f64) * t58225 + F::cast_from(0.27385555555555555555e0_f64) * t68583 + F::cast_from(0.5477111111111111111e0_f64) * t68585 - F::cast_from(0.91285185185185185184e-1_f64) * t68590;
    (t81539, t81552)
}
