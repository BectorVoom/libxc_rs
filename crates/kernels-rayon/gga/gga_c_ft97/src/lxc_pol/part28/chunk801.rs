//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 801/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk801(t590: f64, t7312: f64, t7369: f64, t32888: f64, t7239: f64, t32063: f64, t7366: f64, t7370: f64, t5889: f64, t631: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32889 = t7312 * t590;
    let t32890 = t7369 * t32889;
    let t32892 = t32888 * t7239 * t32890;
    let t32895 = t7366 * t32063 * t7370;
    let t32896 = 2.0_f64 / 3.0_f64 * t32895;
    let t32897 = t5889 * t631;
    (t32889, t32890, t32892, t32895, t32896, t32897)
}
