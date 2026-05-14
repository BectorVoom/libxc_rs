//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 679/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk679<F: Float>(t1392: F, t940: F, t238: F, t242: F, t343: F, t3470: F, t2457: F, t2493: F, t2503: F, t2505: F, t3461: F, t3472: F, t3486: F, t3491: F, t3497: F, t3499: F, t3503: F) -> (F, F, F, F, F) {
    let t3505 = t940 * t1392;
    let t3507 = t238 * t242 * t3505;
    let t3509 = t343 * t3470;
    let t3511 = t238 * t242 * t3509;
    let t3513 = -0.9494625e0 * t3486 + 0.1898925e1 * t3491 + t2493 - 0.29896666666666666667e0 * t2457 - 0.29896666666666666667e0 * t3461 + 0.8969e0 * t3472 + 0.15358125e0 * t3497 + 0.3071625e0 * t3499 + t2503 - 0.16431333333333333333e0 * t2505 - 0.16431333333333333333e0 * t3503 + 0.24647e0 * t3507 + 0.24647e0 * t3511;
    (t3505, t3507, t3509, t3511, t3513)
}
