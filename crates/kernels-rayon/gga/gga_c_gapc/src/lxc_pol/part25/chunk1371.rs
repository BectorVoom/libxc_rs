//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1371/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1371(t33510: f64, t33513: f64, t33518: f64, t33528: f64, t33532: f64, t33536: f64, t33547: f64, t33555: f64, t33558: f64, t33561: f64, t33563: f64, t33567: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36615 = 0.63350674672043801542e-5_f64 * t33510;
    let t36616 = 0.49520679385353736436e-5_f64 * t33513;
    let t36617 = 0.96681162811134562538e-8_f64 * t33518;
    let t36618 = 0.28198672486580914074e-8_f64 * t33528;
    let t36619 = 0.57920616843011475696e-5_f64 * t33532;
    let t36621 = 0.44197102999375800017e-7_f64 * t33536;
    let t36623 = 0.50083268227528753081e-5_f64 * t33547;
    let t36625 = 0.6070699179094394313e-6_f64 * t33555;
    let t36626 = 0.10793703140429833089e-5_f64 * t33558;
    let t36627 = 0.64085799349094910026e-6_f64 * t33561;
    let t36628 = 0.64085799349094910026e-6_f64 * t33563;
    let t36630 = 0.54924190264999682021e-4_f64 * t33567;
    (t36615, t36616, t36617, t36618, t36619, t36621, t36623, t36625, t36626, t36627, t36628, t36630)
}
