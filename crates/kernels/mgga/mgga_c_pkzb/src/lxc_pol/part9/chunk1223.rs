//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1223/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1223<F: Float>(t21001: F, t21004: F, t21006: F, t21008: F, t21010: F, t21012: F, t21014: F, t21016: F, t21018: F, t21021: F, t21024: F, t1954: F, t723: F, t730: F, t7474: F) -> (F, F) {
    let t21287 = -t21001 - t21004 - t21006 - t21008 - t21010 + t21012 + t21014 + t21016 + t21018 - t21021 - t21024;
    let t21291 = F::cast_from(0.35089341735807877242e1_f64) * t730 * t1954 * t7474 * t723;
    (t21287, t21291)
}
