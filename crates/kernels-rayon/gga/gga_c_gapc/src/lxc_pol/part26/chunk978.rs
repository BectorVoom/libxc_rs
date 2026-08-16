//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 978/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk978(t11872: f64, t3408: f64, t1936: f64, t7073: f64, t1453: f64, t291: f64, t7949: f64, t959: f64, t2767: f64, t3717: f64, t7294: f64, t11365: f64, t2660: f64, t7880: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11873 = t11872 * t3408;
    let t11875 = t7073 * t1936;
    let t11876 = t1453 * t291;
    let t11878 = t11876 * t959 * t7949;
    let t11879 = t11875 * t11878;
    let t11882 = t7294 * t3717 * t2767;
    let t11885 = t2660 * t11365 * t7880;
    (t11873, t11875, t11878, t11879, t11882, t11885)
}
