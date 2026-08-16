//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2345/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2345(t1799: f64, t3792: f64, t6414: f64, t1484: f64, t2632: f64, t5611: f64, t154: f64, t2558: f64, t10: f64, t2229: f64, t116: f64, t117: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20468 = t3792 * t1799;
    let t20473 = t3792 * t6414;
    let t20981 = t2632 * t1484;
    let t20986 = t2632 * t5611;
    let t22715 = t2558 * t154;
    let t22811 = t2229 * t10;
    let t22815 = t117 * t116;
    (t20468, t20473, t20981, t20986, t22715, t22811, t22815)
}
