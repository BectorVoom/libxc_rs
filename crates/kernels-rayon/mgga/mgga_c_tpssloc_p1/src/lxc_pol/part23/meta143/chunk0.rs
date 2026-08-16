//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 679/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk679(t340: f64, t5836: f64, t343: f64, t974: f64, t1597: f64) -> (f64, f64, f64) {
    let t5837 = t340 * t5836;
    let t5838 = t5837 * t343;
    let t5839 = t974 * t5838;
    let t5842 = t1597 * t1597;
    (t5838, t5839, t5842)
}
