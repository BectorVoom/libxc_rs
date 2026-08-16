//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 675/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk675(t3469: f64, t457: f64, t460: f64, t974: f64, t1184: f64) -> (f64, f64, f64) {
    let t3470 = t457 * t3469;
    let t3471 = t3470 * t460;
    let t3472 = t974 * t3471;
    let t3475 = t1184 * t1184;
    (t3471, t3472, t3475)
}
