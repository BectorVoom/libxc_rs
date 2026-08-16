//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1364/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1364(t27188: f64, t6525: f64, t1874: f64, t92090: f64, t33603: f64, t6876: f64, t31297: f64, t7685: f64, t191: f64, t192: f64, t27215: f64, t2020: f64) -> (f64, f64, f64, f64, f64) {
    let t121199 = 2.0_f64 * t27188 * t6525;
    let t121201 = 2.0_f64 * t92090 * t1874;
    let t121203 = 3.0_f64 * t6876 * t33603;
    let t121204 = t7685 * t31297;
    let t121210 = t27215 * t191 * t192;
    let t121211 = t121210 * t2020;
    (t121199, t121201, t121203, t121204, t121211)
}
