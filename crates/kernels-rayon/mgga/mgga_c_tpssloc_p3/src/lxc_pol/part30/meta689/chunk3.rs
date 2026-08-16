//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2198/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2198(t26161: f64, t26163: f64, t97920: f64, t24991: f64, t7685: f64, t22574: f64, t25988: f64, t33136: f64, t28823: f64, t6876: f64, t1874: f64, t96709: f64) -> (f64, f64, f64, f64, f64) {
    let t97923 = 4.0_f64 * t26161 * t97920 * t26163;
    let t97925 = 6.0_f64 * t7685 * t24991;
    let t97928 = 6.0_f64 * t22574 * t33136 * t25988;
    let t97930 = 2.0_f64 * t6876 * t28823;
    let t97932 = 2.0_f64 * t96709 * t1874;
    (t97923, t97925, t97928, t97930, t97932)
}
