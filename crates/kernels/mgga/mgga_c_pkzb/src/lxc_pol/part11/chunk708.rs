//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 708/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk708<F: Float>(t110: F, t1564: F, t1569: F, t1572: F, t1582: F, t1590: F, t1608: F, t1615: F, t1618: F, t1622: F, t204: F, t465: F, t49: F, t4902: F, t5019: F, t5022: F, t5025: F, t5028: F, t5029: F, t5040: F, t5044: F, t5048: F, t5052: F, t5056: F, t5066: F, t5069: F, t5073: F, t527: F, t542: F) -> F {
    let t5074 = F::cast_from(0.16562821945185185185e-2_f64) * t49 * t4902 * t110 - t5019 + t5022 - t5025 - t5028 + F::cast_from(0.32530743900905219526e-1_f64) * t204 * t5029 * t1615 + F::cast_from(0.10274e0_f64) * t204 * t465 * t1569 * t1572 - t5040 - F::cast_from(0.51369999999999999999e-1_f64) * t204 * t1564 * t1582 - F::cast_from(0.16522625736956710527e1_f64) * t204 * t5044 * t1590 + F::cast_from(0.68493333333333333332e-1_f64) * t204 * t5048 * t527 - F::cast_from(0.48159733137676571078e0_f64) * t204 * t5052 * t1622 + F::cast_from(0.21687162600603479684e-1_f64) * t204 * t5056 * t542 - F::cast_from(0.16265371950452609763e-1_f64) * t204 * t1608 * t1618 - t5066 + t5069 + t5073;
    t5074
}
