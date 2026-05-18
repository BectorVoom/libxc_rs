//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 665/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk665<F: Float>(t11576: F, t836: F, t568: F, t10010: F, t11834: F, t11837: F, t11841: F, t11845: F, t11849: F, t11854: F, t2028: F, t2103: F, t2197: F, t3651: F, t3677: F, t3681: F, t5748: F, t5775: F, t5782: F, t6148: F, t807: F, t833: F) -> F {
    let t11861 = t836 * t11576;
    let t11862 = t568 * t11861;
    let t11866 = F::new(0.27606906686822939767e2) * t5748 * t11834 + F::new(0.23005755572352449806e1) * t807 * t11837 + F::new(0.69017266717057349418e1) * t6148 * t11841 - F::new(0.39722766613167140743e-1) * t11845 * t2028 - F::new(0.39722766613167140743e-1) * t11849 * t2028 - F::new(0.7150097990370085334e0) * t3651 * t5775 + F::new(0.47667319935800568892e0) * t2103 * t11854 - F::new(0.69017266717057349418e1) * t5782 * t3677 + F::new(0.23005755572352449806e1) * t2197 * t3681 + F::new(0.23005755572352449806e1) * t833 * t11862 - F::new(0.63904876589867916126e-1) * t10010;
    t11866
}
