//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 712/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk712(t457: f64, t6138: f64, t460: f64, t974: f64, t1714: f64) -> (f64, f64, f64) {
    let t6139 = t457 * t6138;
    let t6140 = t6139 * t460;
    let t6141 = t974 * t6140;
    let t6144 = t1714 * t1714;
    (t6140, t6141, t6144)
}
