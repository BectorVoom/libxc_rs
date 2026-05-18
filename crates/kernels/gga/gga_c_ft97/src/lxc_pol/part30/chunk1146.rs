//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1146/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1146<F: Float>(t1466: F, t36048: F, t681: F, t142677: F, t142688: F, t1479: F, t1506: F, t153550: F, t153553: F, t153555: F, t153558: F, t153560: F, t153567: F, t153621: F, t153664: F, t193: F, t2: F, t26: F, t29040: F, t35799: F, t4: F, t6210: F, t6391: F, t7022: F) -> F {
    let t153672 = t1466 * t681 * t36048;
    let t153674 = F::new(8.0) * t153550 + t6210 * t35799 + t153553 / F::new(9.0) + F::new(4.0) * t153555 + F::new(4.0) * t153558 + F::new(4.0) * t153560 + F::new(2.0) / F::new(9.0) * t142677 + t1466 * t193 * t29040 * t1506 / F::new(3.0) + t142688 - t153567 / F::new(18.0) + t1466 * t193 * t7022 * t6391 / F::new(3.0) + (t153621 + t153664) * t2 * t4 * t26 * t1479 / F::new(6.0) - t153672 / F::new(9.0);
    t153674
}
