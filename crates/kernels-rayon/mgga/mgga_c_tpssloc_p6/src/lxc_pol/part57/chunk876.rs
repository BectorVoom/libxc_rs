//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 876/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk876(t7940: f64, t8643: f64, t1983: f64, t25224: f64, t8547: f64, t1880: f64, t1484: f64, t31376: f64, t6637: f64, t6552: f64, t232: f64, t26656: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33366 = t7940 * t8643;
    let t33367 = t1983 * t33366;
    let t33371 = t25224 * t8547;
    let t33372 = t1880 * t33371;
    let t33375 = t31376 * t1484;
    let t33376 = t6637 * t33375;
    let t33377 = t6552 * t33376;
    let t33379 = t26656 * t232;
    (t33366, t33367, t33371, t33372, t33375, t33376, t33377, t33379)
}
