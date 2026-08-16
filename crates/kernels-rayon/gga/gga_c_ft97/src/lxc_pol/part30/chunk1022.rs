//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1022/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1022(t230: f64, t3817: f64, t420: f64, t226: f64, t35371: f64, t1127: f64, t140919: f64, t3762: f64, t1613: f64, t213: f64, t6793: f64, t27729: f64, t9: f64) -> (f64, f64, f64, f64, f64) {
    let t150496 = t420 * t230 * t3817;
    let t150500 = t35371 * t226;
    let t150511 = t140919 * t1127;
    let t150512 = t150511 * t3762;
    let t150516 = t1613 * t6793 * t213;
    let t150517 = t150516 * t3762;
    let t150522 = t27729 * t9;
    (t150496, t150500, t150512, t150517, t150522)
}
