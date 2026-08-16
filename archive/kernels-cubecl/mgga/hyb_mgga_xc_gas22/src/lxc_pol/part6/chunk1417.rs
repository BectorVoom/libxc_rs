//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1417/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1417<F: Float>(t1535: F, t5471: F, t2922: F, t30599: F, t26728: F, t26973: F, t26976: F, t2868: F, t2881: F, t30603: F, t30604: F, t30607: F, t30611: F, t30617: F, t30642: F, t9646: F, t9650: F, t9657: F, t9660: F) -> (F, F, F) {
    let t30657 = t5471 * t1535;
    let t30665 = t2922 * t30657;
    let t30670 = t2922 * t30599;
    let t30681 = -F::cast_from(256.0_f64) / F::cast_from(27.0_f64) * t2881 * t30611 * t9657 - F::cast_from(1600.0_f64) / F::cast_from(27.0_f64) * t26973 * t30617 - F::cast_from(128.0_f64) / F::cast_from(9.0_f64) * t30604 * t9660 - F::cast_from(1280.0_f64) / F::cast_from(27.0_f64) * t2868 * t30657 * t9646 + F::cast_from(8000.0_f64) / F::cast_from(27.0_f64) * t26976 * t30642 - F::cast_from(640.0_f64) / F::cast_from(9.0_f64) * t30607 * t9650 + F::cast_from(512.0_f64) / F::cast_from(9.0_f64) * t30665 * t9657 - F::cast_from(3200.0_f64) / F::cast_from(9.0_f64) * t26728 * t30617 + F::cast_from(256.0_f64) / F::cast_from(3.0_f64) * t30670 * t9660 + F::cast_from(512.0_f64) / F::cast_from(9.0_f64) * t2922 * t30611 * t9646 + F::cast_from(3200.0_f64) / F::cast_from(9.0_f64) * t26728 * t30642 + F::cast_from(256.0_f64) / F::cast_from(3.0_f64) * t2922 * t30603 * t9650;
    (t30657, t30670, t30681)
}
