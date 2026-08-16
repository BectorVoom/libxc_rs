//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 994/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk994(t10002: f64, t35639: f64, t27915: f64, t7437: f64, t2568: f64, t35678: f64, t766: f64, t1403: f64, t35286: f64, t681: f64, t10157: f64, t140707: f64, t27836: f64, t27894: f64, t27958: f64, t28467: f64, t33255: f64, t33575: f64, t35282: f64, t5996: f64, t6002: f64, t6003: f64, t6745: f64, t7491: f64) -> (f64, f64, f64) {
    let t149997 = t10002 * t35639;
    let t150009 = t7437 * t27915;
    let t150014 = t2568 * t35678 * t766;
    let t150017 = t1403 * t681 * t35286;
    let t150020 = 2.0_f64 * t6002 * t10157 * t6003 * t27836 + 4.0_f64 * t149997 - t7437 * t27958 / 3.0_f64 - t7437 * t28467 / 3.0_f64 + t5996 * t35282 / 6.0_f64 - t6745 * t33255 / 3.0_f64 - t6745 * t33575 / 3.0_f64 - t150009 / 18.0_f64 + t27894 * t7491 / 3.0_f64 + 4.0_f64 * t150014 + 2.0_f64 / 9.0_f64 * t150017 + t140707 / 9.0_f64;
    (t149997, t150014, t150020)
}
