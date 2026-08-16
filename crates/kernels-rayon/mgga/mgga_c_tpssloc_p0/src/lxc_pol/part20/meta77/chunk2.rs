//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 555/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk555(t1603: f64, t381: f64, t1409: f64, t998: f64, t974: f64, t225: f64) -> (f64, f64, f64, f64) {
    let t1604 = t1603 * t381;
    let t1606 = t998 * t1409;
    let t1607 = t974 * t1606;
    let t1610 = t1603 * t225;
    (t1604, t1606, t1607, t1610)
}
