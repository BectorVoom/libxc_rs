//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1429/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1429(t22581: f64, t6876: f64, t191: f64, t192: f64, t9419: f64, t2020: f64, t12451: f64, t3701: f64, t1983: f64, t2019: f64, t1874: f64, t45640: f64) -> (f64, f64, f64, f64) {
    let t83896 = 6.0_f64 * t6876 * t22581;
    let t83904 = t9419 * t191 * t192;
    let t83905 = t83904 * t2020;
    let t83911 = t3701 * t12451;
    let t83913 = t1983 * t2019 * t83911;
    let t83917 = 2.0_f64 * t45640 * t1874;
    (t83896, t83905, t83913, t83917)
}
