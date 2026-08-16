//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1427/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1427(t22574: f64, t39367: f64, t8643: f64, t22607: f64, t6997: f64, t12156: f64, t1390: f64, t1983: f64, t2018: f64, t22597: f64, t6876: f64, t22585: f64) -> (f64, f64, f64, f64, f64) {
    let t83869 = 9.0_f64 * t22574 * t8643 * t39367;
    let t83876 = 3.0_f64 * t22607 * t6997;
    let t83880 = 6.0_f64 * t1983 * t12156 * t2018 * t1390;
    let t83882 = 18.0_f64 * t6876 * t22597;
    let t83884 = 9.0_f64 * t6876 * t22585;
    (t83869, t83876, t83880, t83882, t83884)
}
