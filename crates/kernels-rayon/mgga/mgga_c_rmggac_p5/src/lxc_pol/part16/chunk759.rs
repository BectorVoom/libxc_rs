//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 759/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk759(t35620: f64, t7558: f64, t290: f64, t35604: f64, t2010: f64, t7755: f64, t1341: f64, t303: f64, t638: f64, t7310: f64, t357: f64, t7254: f64, t7364: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35621 = t35620 * t7558;
    let t35623 = t290 * t35604;
    let t35625 = t2010 * t7755 * t35623;
    let t35629 = t638 * t7310 * t303 * t1341;
    let t35633 = t638 * t7310 * t357 * t1341;
    let t35637 = t7254 * t7364;
    (t35621, t35623, t35625, t35629, t35633, t35637)
}
