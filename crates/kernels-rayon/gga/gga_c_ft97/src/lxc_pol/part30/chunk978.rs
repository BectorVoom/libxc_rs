//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 978/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk978(t668: f64, t7679: f64, t7672: f64, t34199: f64, t8392: f64, t34160: f64, t34209: f64, t1882: f64, t34091: f64, t34115: f64, t34232: f64, t7631: f64, t8232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t143660 = t7679 * t668;
    let t143673 = t7672 * t668;
    let t143718 = t8392 * t34199;
    let t143720 = t8392 * t34160;
    let t143722 = t8392 * t34209;
    let t143753 = t1882 * t34091;
    let t143789 = t1882 * t34115;
    let t143823 = t1882 * t34232;
    let t143858 = 8.0_f64 / 27.0_f64 * t8232 * t7631;
    (t143660, t143673, t143718, t143720, t143722, t143753, t143789, t143823, t143858)
}
