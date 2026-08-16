//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 287/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk287(t3902: f64, t747: f64, t91: f64, t1148: f64, t1775: f64, t2: f64, t2486: f64, t3691: f64, t2493: f64, t3695: f64, t737: f64, t3700: f64) -> (f64, f64, f64, f64, f64) {
    let t3904 = t91 * t3902 * t747;
    let t3908 = t1775 * t1148;
    let t3910 = t2486 * t2;
    let t3911 = t3910 * t3691;
    let t3914 = t2493 * t3695;
    let t3917 = t737 * t2;
    let t3918 = t3917 * t3700;
    (t3904, t3908, t3911, t3914, t3918)
}
