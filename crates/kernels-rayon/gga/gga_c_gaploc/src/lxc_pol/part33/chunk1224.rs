//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1224/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1224(t1980: f64, t8792: f64, t10024: f64, t10843: f64, t2013: f64, t11038: f64, t4614: f64, t813: f64, t10964: f64, t2194: f64, t10717: f64, t833: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32757 = t1980 * t8792;
    let t32758 = t32757 * t10024;
    let t32759 = 0.89376224879626066674e-1_f64 * t32758;
    let t32760 = t2013 * t10843;
    let t32761 = 0.51123901271894332902e0_f64 * t32760;
    let t32764 = 0.12269736305254639897e2_f64 * t813 * t4614 * t11038;
    let t32766 = 0.12269736305254639897e2_f64 * t2194 * t10964;
    let t32769 = 0.30674340763136599742e2_f64 * t833 * t4614 * t10717;
    (t32757, t32759, t32761, t32764, t32766, t32769)
}
