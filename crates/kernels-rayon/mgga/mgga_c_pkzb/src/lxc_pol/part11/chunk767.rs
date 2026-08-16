//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 767/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk767(t1435: f64, t23: f64, t1424: f64, t34: f64, t38: f64, t1020: f64, t568: f64) -> (f64, f64, f64, f64) {
    let t6679 = t23 * t1435;
    let t6723 = t34 * t1424;
    let t6738 = t38 * t1435;
    let t6758 = t1020 * t568;
    (t6679, t6723, t6738, t6758)
}
