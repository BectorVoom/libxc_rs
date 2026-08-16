//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 724/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk724(t8620: f64, t8621: f64, t1044: f64, t1971: f64, t1743: f64, t5722: f64, t458: f64, t4925: f64, t3104: f64, t568: f64, t3108: f64, t1027: f64, t1842: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8622 = t8620 * t8621;
    let t8624 = t1971 * t1044;
    let t8625 = t1743 * t8624;
    let t8626 = t8625 * t5722;
    let t8628 = t4925 * t458;
    let t8629 = t3104 * t8628;
    let t8631 = t4925 * t568;
    let t8632 = t3108 * t8631;
    let t8634 = t1027 * t1842;
    (t8622, t8624, t8626, t8629, t8632, t8634)
}
