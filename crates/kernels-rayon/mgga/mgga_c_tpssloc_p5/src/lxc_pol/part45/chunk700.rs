//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 700/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk700(t191: f64, t192: f64, t3660: f64, t2020: f64, t2314: f64, t6535: f64, t12823: f64, t1874: f64, t4034: f64, t6525: f64, t12734: f64, t671: f64, t6862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22607 = t3660 * t191 * t192;
    let t22608 = t22607 * t2020;
    let t22610 = 4.0_f64 * t2314 * t6535;
    let t22612 = 2.0_f64 * t12823 * t1874;
    let t22614 = 4.0_f64 * t4034 * t6525;
    let t22616 = 4.0_f64 * t12734 * t1874;
    let t22618 = 4.0_f64 * t2314 * t6525;
    let t22619 = t6862 * t671;
    (t22607, t22608, t22610, t22612, t22614, t22616, t22618, t22619)
}
