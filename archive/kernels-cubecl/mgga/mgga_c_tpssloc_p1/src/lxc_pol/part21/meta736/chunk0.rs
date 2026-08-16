//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2596/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2596<F: Float>(t13969: F, t15636: F, t3515: F, t1174: F, t44571: F, t4724: F, t11778: F, t43791: F, t1227: F, t49850: F, t4988: F, t15568: F, t3604: F) -> (F, F, F, F, F) {
    let t52586 = t3515 * t13969 * t15636;
    let t52599 = t1174 * t44571 * t4724;
    let t52601 = t11778 * t43791;
    let t52609 = t1227 * t49850 * t4988;
    let t52615 = t3604 * t15568;
    (t52586, t52599, t52601, t52609, t52615)
}
