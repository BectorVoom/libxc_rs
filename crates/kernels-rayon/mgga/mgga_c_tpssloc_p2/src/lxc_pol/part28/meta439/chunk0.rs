//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1622/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1622(t22690: f64, t6638: f64, t23171: f64, t828: f64, t852: f64, t232: f64, t6646: f64, t1888: f64, t10097: f64, t206: f64, t268: f64, t6559: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23172 = t22690 * t6638;
    let t23173 = t23171 * t23172;
    let t23175 = t852 * t828;
    let t23176 = t23175 * t232;
    let t23177 = t6646 * t23176;
    let t23178 = t1888 * t23177;
    let t23180 = t10097 * t232;
    let t23181 = t6646 * t23180;
    let t23182 = t1888 * t23181;
    let t23185 = t6559 * t206 * t268;
    (t23172, t23173, t23176, t23177, t23178, t23180, t23181, t23182, t23185)
}
