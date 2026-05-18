//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1224/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1224<F: Float>(t1976: F, t2874: F, t730: F, t7474: F, t1987: F, t7532: F, t1107: F, t17474: F, t17478: F, t5484: F, t2860: F, t5486: F) -> (F, F, F, F) {
    let t21299 = F::new(0.51947577317044391277e2) * t730 * t1976 * t7474 * t2874;
    let t21301 = F::new(0.31168546390226634765e3) * t1987 * t7532;
    let t21306 = F::new(0.91082604192152556044e5) * t730 * t17474 * t1107 * t17478 * t5484;
    let t21308 = F::new(0.35089341735807877242e1) * t2860 * t5486;
    (t21299, t21301, t21306, t21308)
}
