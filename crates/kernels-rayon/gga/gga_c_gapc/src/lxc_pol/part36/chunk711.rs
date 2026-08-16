//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 711/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk711(t1567: f64, t2890: f64, t8442: f64, t1001: f64, t8422: f64, t1901: f64) -> (f64, f64, f64) {
    let t8443 = t2890 * t1567;
    let t8444 = t8442 * t8443;
    let t8446 = t8422 * t1001;
    let t8448 = 1.0_f64 / t1901;
    (t8444, t8446, t8448)
}
