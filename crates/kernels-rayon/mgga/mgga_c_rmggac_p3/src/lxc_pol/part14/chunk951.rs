//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 951/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk951(t1986: f64, t2318: f64, t326: f64, t333: f64, t7717: f64, t236: f64, t321: f64, t7230: f64, t7248: f64, t8666: f64, t551: f64, t7817: f64) -> (f64, f64, f64) {
    let t40323 = t1986 * t326 * t2318 * t333;
    let t40324 = t7717 * t40323;
    let t40329 = t7230 * t7248 * t236 * t8666 * t321;
    let t40331 = t7817 * t551;
    (t40324, t40329, t40331)
}
