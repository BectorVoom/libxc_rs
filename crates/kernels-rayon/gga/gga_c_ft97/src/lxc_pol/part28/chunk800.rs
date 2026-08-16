//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 800/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk800(t32869: f64, t525: f64, t165: f64, t28: f64, t614: f64, t7340: f64, t1359: f64, t1389: f64, t1969: f64, t379: f64, t170: f64, t23608: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32870 = t525 * t32869;
    let t32871 = t32870 * t165;
    let t32872 = t28 * t32871;
    let t32875 = t7340 * t614;
    let t32876 = t28 * t32875;
    let t32879 = t1359 * t1389;
    let t32881 = t1969 * t32879 * t379;
    let t32888 = t23608 * t170;
    (t32870, t32871, t32872, t32875, t32876, t32879, t32881, t32888)
}
