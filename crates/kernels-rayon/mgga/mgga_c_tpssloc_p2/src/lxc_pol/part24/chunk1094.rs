//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1094/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1094(t2267: f64, t38: f64, t240: f64, t2244: f64, t2250: f64, t22502: f64, t2261: f64, t44: f64, t607: f64, t6500: f64, t67: f64, t1864: f64) -> (f64, f64, f64, f64) {
    let t22505 = t38 * t2267;
    let t22510 = 88.0_f64 / 9.0_f64 * t240;
    let t22511 = 88.0_f64 / 9.0_f64 * t2261 * t44 - 40.0_f64 / 9.0_f64 * t22502 * t607 + 5.0_f64 / 18.0_f64 * t22505 * t2244 + 5.0_f64 / 6.0_f64 * t6500 * t2250 - t22510;
    let t22512 = t22511 * t67;
    let t22513 = t22512 * t1864;
    (t22505, t22511, t22512, t22513)
}
