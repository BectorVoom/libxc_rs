//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 867/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk867(t3352: f64, t352: f64, t515: f64, t7230: f64, t8829: f64, t1986: f64, t2318: f64, t305: f64, t321: f64, t7717: f64, t1981: f64, t512: f64, t676: f64, t8512: f64) -> (f64, f64, f64) {
    let t39099 = t7230 * t3352 * t515 * t8829 * t352;
    let t39103 = t1986 * t305 * t2318 * t321;
    let t39104 = t7717 * t39103;
    let t39108 = t8512 * t1981 * t676 * t512;
    (t39099, t39104, t39108)
}
