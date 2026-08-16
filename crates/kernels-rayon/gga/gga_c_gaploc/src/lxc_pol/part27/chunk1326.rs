//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1326/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1326(t1415: f64, t1646: f64, t34600: f64, t2299: f64, t2754: f64, t10319: f64, t4762: f64, t10318: f64, t4398: f64, t26609: f64, t6628: f64, t6798: f64, t8411: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34603 = 0.71500979903700853338e0_f64 * t1415 * t34600 * t1646;
    let t34604 = t2299 * t2754;
    let t34607 = 0.71500979903700853338e0_f64 * t1415 * t34604 * t1646;
    let t34609 = 0.35750489951850426669e0_f64 * t10319 * t4762;
    let t34612 = 0.71500979903700853338e0_f64 * t4398 * t10318 * t1646;
    let t34614 = 0.21450293971110256002e1_f64 * t26609 * t6628;
    let t34621 = 0.14300195980740170668e1_f64 * t8411 * t6798;
    (t34603, t34607, t34609, t34612, t34614, t34621)
}
