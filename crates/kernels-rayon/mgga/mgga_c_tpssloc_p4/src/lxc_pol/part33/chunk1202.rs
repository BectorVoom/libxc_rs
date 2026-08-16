//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1202/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1202(t26167: f64, t7687: f64, t1983: f64, t191: f64, t192: f64, t6295: f64, t2020: f64, t20085: f64, t2019: f64, t1390: f64, t6330: f64, t22595: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28817 = t26167 * t7687;
    let t28819 = 6.0_f64 * t1983 * t28817;
    let t28821 = t6295 * t191 * t192;
    let t28822 = t28821 * t2020;
    let t28823 = t2019 * t20085;
    let t28825 = 2.0_f64 * t1983 * t28823;
    let t28826 = t1390 * t6330;
    let t28827 = t22595 * t28826;
    (t28817, t28819, t28821, t28822, t28823, t28825, t28826, t28827)
}
