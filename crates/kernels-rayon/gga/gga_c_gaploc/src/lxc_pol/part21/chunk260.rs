//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 260/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk260(t913: f64, t969: f64, t825: f64, t836: f64, t935: f64, t568: f64, t317: f64, t797: f64, t813: f64, t833: f64, t955: f64, t960: f64, t962: f64, t966: f64) -> (f64, f64, f64, f64, f64) {
    let t970 = t969 * t913;
    let t971 = t825 * t970;
    let t973 = t836 * t935;
    let t974 = t568 * t973;
    let t977 = 0.35750489951850426669e0_f64 * t955 * t317 + 0.14896037479937677779e-1_f64 * t960 - 0.35750489951850426669e0_f64 * t797 * t962 - 0.23005755572352449806e1_f64 * t813 * t966 - 0.95857314884801874192e-1_f64 * t971 + 0.23005755572352449806e1_f64 * t833 * t974;
    (t970, t971, t973, t974, t977)
}
