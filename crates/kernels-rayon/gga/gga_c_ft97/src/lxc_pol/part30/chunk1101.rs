//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1101/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1101(t1476: f64, t28719: f64, t1486: f64, t193: f64, t2781: f64, t33978: f64, t4255: f64, t10248: f64, t446: f64, t4129: f64, t7584: f64, t10570: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t152767 = t1476 * t28719;
    let t152770 = t1486 * t193 * t2781 * t152767;
    let t152772 = t33978 * t4255;
    let t152774 = t446 * t10248 * t152772;
    let t152776 = t7584 * t4129;
    let t152779 = t1486 * t193 * t10570 * t152776;
    (t152767, t152770, t152772, t152774, t152776, t152779)
}
