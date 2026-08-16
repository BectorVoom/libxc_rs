//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2348/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2348(t5415: f64, t55: f64, t16558: f64, t17635: f64, t17686: f64, t17691: f64, t1860: f64, t1864: f64, t24498: f64, t26090: f64, t27311: f64, t27332: f64, t27356: f64, t27364: f64, t29474: f64, t29475: f64, t29478: f64, t29481: f64, t3961: f64, t3966: f64, t607: f64, t6486: f64, t6495: f64, t6509: f64, t67: f64, t7246: f64, t7251: f64, t7428: f64, t7432: f64, t7445: f64, t83803: f64, t85539: f64, t96025: f64, t96157: f64, t96393: f64) -> f64 {
    let t104818 = t5415 * t55;
    let t104858 = -t7428 * t27311 / 3.0_f64 - t6486 * t29475 / 6.0_f64 - t1860 * (-220.0_f64 / 27.0_f64 * t104818 * t607 - 40.0_f64 / 27.0_f64 * t96157 * t3961 + 40.0_f64 / 9.0_f64 * t27356 * t3966 + 5.0_f64 / 108.0_f64 * t85539 * t17686 + 5.0_f64 / 9.0_f64 * t24498 * t17691 + 5.0_f64 / 18.0_f64 * t24498 * t17635 - 5.0_f64 / 6.0_f64 * t7251 * t16558 + t83803) * t67 * t1864 / 6.0_f64 - t1860 * t29474 * t6509 / 6.0_f64 - t6486 * t29478 / 3.0_f64 - t1860 * t27364 * t7445 / 3.0_f64 + t6495 * t29475 / 3.0_f64 + 5.0_f64 / 3.0_f64 * t27332 * t26090 + 2.0_f64 / 3.0_f64 * t6495 * t29478 + 5.0_f64 / 6.0_f64 * t7246 * t96393 + t6495 * t29481 / 3.0_f64 + 5.0_f64 / 3.0_f64 * t96025 * t7432;
    t104858
}
