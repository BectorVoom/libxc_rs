//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 794/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk794(t33978: f64, t505: f64, t2665: f64, t446: f64, t2680: f64, t7611: f64) -> (f64, f64, f64) {
    let t33979 = t33978 * t505;
    let t33980 = t2665 * t33979;
    let t33981 = t446 * t33980;
    let t33983 = t2680 * t7611;
    (t33980, t33981, t33983)
}
