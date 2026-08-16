//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1423/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1423(t835: f64, t10913: f64, t1860: f64, t1864: f64, t2244: f64, t22490: f64, t2250: f64, t22502: f64, t22505: f64, t22512: f64, t22513: f64, t22516: f64, t22534: f64, t22551: f64, t44: f64, t607: f64, t6486: f64, t6490: f64, t6492: f64, t6495: f64, t6500: f64, t6506: f64, t6509: f64, t6510: f64, t67: f64, t83771: f64, t83775: f64, t83778: f64, t83788: f64, t83791: f64, t83796: f64, t9258: f64, t9276: f64, t9288: f64) -> f64 {
    let t83803 = 1232.0_f64 / 27.0_f64 * t835;
    let t83812 = 2.0_f64 * t6495 * t22516 + 5.0_f64 / 2.0_f64 * t6490 * t83771 + t6495 * t22490 + 5.0_f64 / 2.0_f64 * t83775 * t6492 - 5.0_f64 * t83778 * t22551 + t22534 * t6506 + t22534 * t6510 - t6486 * t22513 / 2.0_f64 - t6486 * t22516 - t1860 * (-1232.0_f64 / 27.0_f64 * t9276 * t44 + 220.0_f64 / 9.0_f64 * t83788 * t607 - 20.0_f64 / 9.0_f64 * t83791 * t2244 - 20.0_f64 / 3.0_f64 * t22502 * t2250 - 5.0_f64 / 108.0_f64 * t83796 * t9288 + 5.0_f64 / 6.0_f64 * t22505 * t10913 + 5.0_f64 / 6.0_f64 * t6500 * t9258 + t83803) * t67 * t1864 / 6.0_f64 - t1860 * t22512 * t6509 / 2.0_f64;
    t83812
}
