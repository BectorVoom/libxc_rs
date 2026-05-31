//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 692/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk692<F: Float>(t1392: F, t940: F, t238: F, t242: F, t343: F, t3470: F, t2457: F, t2493: F, t2503: F, t2505: F, t3461: F, t3472: F, t3486: F, t3491: F, t3497: F, t3499: F, t3503: F) -> (F, F, F, F, F) {
    let t3505 = t940 * t1392;
    let t3507 = t238 * t242 * t3505;
    let t3509 = t343 * t3470;
    let t3511 = t238 * t242 * t3509;
    let t3513 = -F::cast_from(0.9494625e0_f64) * t3486 + F::cast_from(0.1898925e1_f64) * t3491 + t2493 - F::cast_from(0.29896666666666666667e0_f64) * t2457 - F::cast_from(0.29896666666666666667e0_f64) * t3461 + F::cast_from(0.8969e0_f64) * t3472 + F::cast_from(0.15358125e0_f64) * t3497 + F::cast_from(0.3071625e0_f64) * t3499 + t2503 - F::cast_from(0.16431333333333333333e0_f64) * t2505 - F::cast_from(0.16431333333333333333e0_f64) * t3503 + F::cast_from(0.24647e0_f64) * t3507 + F::cast_from(0.24647e0_f64) * t3511;
    (t3505, t3507, t3509, t3511, t3513)
}
