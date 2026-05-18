//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 526/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk526<F: Float>(t940: F, t238: F, t242: F, t2466: F, t343: F, t2457: F, t2468: F, t2486: F, t2491: F, t2493: F, t2497: F, t2499: F, t2503: F, t2505: F) -> (F, F, F, F, F) {
    let t2507 = t940 * t940;
    let t2509 = t238 * t242 * t2507;
    let t2511 = t343 * t2466;
    let t2513 = t238 * t242 * t2511;
    let t2515 = -F::new(0.9494625e0) * t2486 + F::new(0.1898925e1) * t2491 + t2493 - F::new(0.59793333333333333334e0) * t2457 + F::new(0.8969e0) * t2468 + F::new(0.15358125e0) * t2497 + F::new(0.3071625e0) * t2499 + t2503 - F::new(0.32862666666666666666e0) * t2505 + F::new(0.24647e0) * t2509 + F::new(0.24647e0) * t2513;
    (t2507, t2509, t2511, t2513, t2515)
}
