//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 714/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk714(t433: f64, t463: f64, t1567: f64, t2890: f64, t1001: f64, t8422: f64, t1901: f64) -> (f64, f64, f64) {
    let t8442 = t463 * t433;
    let t8443 = t2890 * t1567;
    let t8444 = t8442 * t8443;
    let t8446 = t8422 * t1001;
    let t8448 = 1.0_f64 / t1901;
    (t8444, t8446, t8448)
}
