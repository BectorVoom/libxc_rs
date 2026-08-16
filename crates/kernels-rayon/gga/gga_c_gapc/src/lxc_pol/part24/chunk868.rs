//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 868/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk868(t2300: f64, t493: f64, t3217: f64, t1061: f64, t6925: f64, t3239: f64, t6927: f64, t8292: f64, t8298: f64, t8301: f64, t8304: f64, t8306: f64, t8311: f64, t8314: f64, t8317: f64, t8319: f64, t8322: f64, t8324: f64) -> (f64, f64, f64) {
    let t10398 = t493 * t2300;
    let t10399 = t3217 * t10398;
    let t10401 = t1061 * t6925;
    let t10402 = t3239 * t6927;
    let t10403 = t10401 * t10402;
    let t10433 = -0.59049582388386525904e-5_f64 * t8292 - 0.59049582388386525904e-5_f64 * t8298 - 0.86898242813537603826e-4_f64 * t8301 + 0.43449121406768801913e-4_f64 * t8304 + 0.20855578275249024918e-2_f64 * t8306 + 0.12147342662753799615e-3_f64 * t8311 + 0.86898242813537603826e-5_f64 * t8314 + 0.86898242813537603826e-5_f64 * t8317 + 0.41711156550498049836e-2_f64 * t8319 - 0.82402707051983925121e-5_f64 * t8322 - 0.60828769969476322678e-4_f64 * t8324;
    (t10399, t10403, t10433)
}
