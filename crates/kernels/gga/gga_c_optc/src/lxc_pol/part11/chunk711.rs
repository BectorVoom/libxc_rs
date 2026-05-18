//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 711/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk711<F: Float>(t256: F, t7501: F, t248: F, t2516: F, t243: F, t7592: F, t7523: F, t808: F, t251: F, t2519: F, t7341: F, t224: F, t2269: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7753 = t256 * t7501;
    let t7758 = F::new(1.0) / t2516 / t248;
    let t7759 = t243 * t7758;
    let t7786 = F::new(0.46308888888888888888e0) * t7592;
    let t7787 = F::new(0.16068111111111111111e1) * t7523;
    let t7798 = F::new(1.0) / t2516 / t808;
    let t7799 = t243 * t7798;
    let t7801 = F::new(1.0) / t2519 / t251;
    let t7813 = t256 * t7341;
    let t7856 = F::new(1.0) / t224 / t2269;
    (t7753, t7758, t7759, t7786, t7787, t7798, t7799, t7801, t7813, t7856)
}
