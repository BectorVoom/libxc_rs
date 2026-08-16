//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 405/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk405(t2179: f64, t5956: f64, t144: f64, t5897: f64, t5914: f64, t5894: f64, t5903: f64, t5907: f64, t5911: f64, t5919: f64, t5923: f64, t5927: f64) -> (f64, f64, f64, f64) {
    let t5957 = t2179 * t5956;
    let t5958 = t144 * t5957;
    let t5962 = t5897 / 6.0_f64;
    let t5965 = t5914 / 3.0_f64;
    let t5968 = t5894 / 4.0_f64 + t5962 + t5903 / 6.0_f64 + t5907 - t5911 / 2.0_f64 + t5965 + t5919 / 3.0_f64 + 2.0_f64 * t5923 - t5927;
    (t5958, t5962, t5965, t5968)
}
