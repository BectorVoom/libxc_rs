//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1107/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1107(t10248: f64, t152844: f64, t446: f64, t193: f64, t35972: f64, t6308: f64, t852: f64, t856: f64, t824: f64, t1486: f64, t2781: f64, t4129: f64, t7611: f64) -> (f64, f64, f64, f64, f64) {
    let t152849 = t446 * t10248 * t152844;
    let t152854 = t6308 * t193 * t852 * t35972 * t856;
    let t152856 = t35972 * t824;
    let t152859 = t1486 * t193 * t2781 * t152856;
    let t152861 = t7611 * t4129;
    (t152849, t152854, t152856, t152859, t152861)
}
