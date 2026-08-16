//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 982/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk982(t2286: f64, t7939: f64, t2412: f64, t7914: f64, t3351: f64, t3352: f64, t5181: f64, t880: f64, t1243: f64, t515: f64, t570: f64, t7231: f64) -> (f64, f64, f64, f64) {
    let t41585 = t7939 * t2286;
    let t41587 = t2412 * t7914;
    let t41591 = t3351 * t3352 * t880 * t5181;
    let t41596 = t3351 * t7231 * t515 * t570 * t1243;
    (t41585, t41587, t41591, t41596)
}
