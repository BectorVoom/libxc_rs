//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 791/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk791(t33953: f64, t852: f64, t1486: f64, t193: f64, t375: f64, t7654: f64, t89: f64, t668: f64, t7584: f64) -> (f64, f64, f64, f64, f64) {
    let t33954 = t852 * t33953;
    let t33956 = t1486 * t193 * t33954;
    let t33959 = t89 * t375 * t7654;
    let t33960 = 2.0_f64 / 9.0_f64 * t33959;
    let t33961 = t7584 * t668;
    (t33954, t33956, t33959, t33960, t33961)
}
