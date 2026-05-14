//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 722/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk722<F: Float>(t10642: F, t10659: F, t295: F, t312: F, t2832: F, t870: F, t875: F, t296: F, t1882: F, t2859: F, t10510: F, t10514: F, t10518: F, t10522: F, t10526: F, t10530: F, t10533: F, t10536: F, t10539: F, t10542: F, t10545: F, t10548: F, t1901: F, t193: F, t446: F, t89: F) -> (F, F, F, F, F, F) {
    let t10660 = t10642 + t10659;
    let t10662 = t295 * t10660 * t312;
    let t10666 = t2832 * t870;
    let t10667 = t10666 * t875;
    let t10668 = t296 * t10667;
    let t10670 = t1882 * t2859;
    let t10672 = -2.0 / 3.0 * t1901 * t10510 + 4.0 / 9.0 * t10514 - 2.0 * t446 * t10518 - t446 * t10522 / 3.0 - 2.0 / 9.0 * t446 * t10526 - t446 * t10530 / 9.0 - 2.0 / 3.0 * t10533 - t446 * t10536 / 3.0 + 2.0 / 9.0 * t10539 - 2.0 / 3.0 * t446 * t10542 - 2.0 / 9.0 * t10545 + 4.0 / 9.0 * t446 * t10548 + t89 * t193 * t10662 / 3.0 - t446 * t10668 + 2.0 / 27.0 * t10670;
    (t10660, t10662, t10666, t10667, t10668, t10672)
}
