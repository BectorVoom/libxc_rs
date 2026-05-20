//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2566/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2566<F: Float>(t487: F, t56331: F, t56176: F, t56183: F, t56228: F, t12627: F, t1811: F, t1269: F, t17306: F, t3565: F, t5215: F, t3566: F, t5412: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t56332 = t56331 * t487;
    let t56343 = F::cast_from(0.13170370370370370371e-1_f64) * t56176;
    let t56345 = F::cast_from(0.39511111111111111112e-1_f64) * t56183;
    let t56360 = F::cast_from(0.19755555555555555556e-1_f64) * t56228;
    let t56393 = t12627 * t1811;
    let t56416 = t17306 * t1269;
    let t56447 = F::cast_from(0.22222222222222222222e-1_f64) * t56183;
    let t56462 = F::cast_from(0.11111111111111111111e-1_f64) * t56228;
    let t56587 = t5215 * t3565;
    let t56588 = t56587 * t487;
    let t56607 = t3566 * t5412;
    (t56332, t56343, t56345, t56360, t56393, t56416, t56447, t56462, t56587, t56588, t56607)
}
