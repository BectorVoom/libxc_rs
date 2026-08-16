//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 788/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk788(t2922: f64, t7716: f64, t2037: f64, t7706: f64, t5953: f64, t7663: f64) -> (f64, f64, f64) {
    let t7718 = 0.28582678745379824648e-3_f64 * t2922 * t7716;
    let t7725 = t2037 * t7706;
    let t7736 = t5953 * t7663;
    (t7718, t7725, t7736)
}
