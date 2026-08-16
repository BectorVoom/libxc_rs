//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 418/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk418(t241: f64, t258: f64, t6907: f64, t1168: f64, t6154: f64, t242: f64, t1091: f64, t6161: f64, t2606: f64, t1131: f64, t1449: f64) -> (f64, f64, f64, f64, f64) {
    let t6909 = t241 * t6907 * t258;
    let t6913 = t6154 * t1168;
    let t6914 = t242 * t6913;
    let t6917 = t6161 * t1091;
    let t6918 = t2606 * t6917;
    let t6921 = t1449 * t1131;
    (t6909, t6914, t6917, t6918, t6921)
}
