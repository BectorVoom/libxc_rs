//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1036/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1036(t191: f64, t192: f64, t24026: f64, t2020: f64, t15904: f64, t22574: f64, t36740: f64, t22579: f64, t8607: f64, t31668: f64, t532: f64, t1983: f64, t6879: f64) -> (f64, f64, f64, f64) {
    let t115765 = t24026 * t191 * t192;
    let t115766 = t115765 * t2020;
    let t115771 = 6.0_f64 * t22574 * t36740 * t15904;
    let t115773 = t8607 * t22579;
    let t115774 = t532 * t31668;
    let t115777 = 6.0_f64 * t1983 * t115774 * t6879;
    (t115766, t115771, t115773, t115777)
}
