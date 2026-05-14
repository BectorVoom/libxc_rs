//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1119/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1119<F: Float>(t1987: F, t7532: F, t1107: F, t17474: F, t17478: F, t5484: F, t730: F, t2860: F, t5486: F, t1306: F, t21027: F, t21030: F, t21033: F, t21037: F, t21039: F, t21291: F, t21299: F, t2153: F, t2993: F, t6065: F) -> (F, F, F, F) {
    let t21301 = 0.31168546390226634765e3 * t1987 * t7532;
    let t21306 = 0.91082604192152556044e5 * t730 * t17474 * t1107 * t17478 * t5484;
    let t21308 = 0.35089341735807877242e1 * t2860 * t5486;
    let t21309 = 6.0 * t1306 * t2153 * t2993 * t6065 - t21027 - t21030 - t21033 + t21037 + t21039 + t21291 - t21299 + t21301 - t21306 - t21308;
    (t21301, t21306, t21308, t21309)
}
