//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2115/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2115(t2235: f64, t5392: f64, t16558: f64, t17635: f64, t17686: f64, t17691: f64, t1860: f64, t1864: f64, t1865: f64, t22502: f64, t22505: f64, t26021: f64, t26024: f64, t26025: f64, t26028: f64, t26044: f64, t26048: f64, t27949: f64, t27950: f64, t27953: f64, t27957: f64, t5398: f64, t6486: f64, t6500: f64, t6509: f64, t67: f64, t7428: f64, t7441: f64, t7445: f64, t7446: f64, t83791: f64, t83796: f64, t83803: f64) -> f64 {
    let t96646 = t2235 * t5392;
    let t96649 = -t7428 * t26048 / 3.0_f64 - t26028 * t7446 / 3.0_f64 - t7428 * t26021 / 3.0_f64 - t7428 * t26025 / 3.0_f64 - t6486 * t27950 / 6.0_f64 - t1860 * (-20.0_f64 / 27.0_f64 * t83791 * t5392 - 5.0_f64 / 108.0_f64 * t83796 * t17686 + 5.0_f64 / 9.0_f64 * t22505 * t17691 - 20.0_f64 / 9.0_f64 * t22502 * t5398 + 5.0_f64 / 18.0_f64 * t22505 * t17635 + 5.0_f64 / 6.0_f64 * t6500 * t16558 + t83803) * t67 * t1864 / 6.0_f64 - t1860 * t27949 * t6509 / 6.0_f64 - t6486 * t27953 / 3.0_f64 - t1860 * t26044 * t7445 / 3.0_f64 - t1860 * t7441 * t26024 / 3.0_f64 - t6486 * t27957 / 6.0_f64 + t96646 * t1865 / 3.0_f64;
    t96649
}
