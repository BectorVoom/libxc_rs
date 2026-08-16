//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 914/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk914(t10136: f64, t10170: f64, t10200: f64, t10240: f64, t10283: f64, t10323: f64, t10370: f64, t10405: f64, t8292: f64, t8298: f64, t8301: f64, t8304: f64, t8306: f64, t8311: f64, t8314: f64, t8317: f64, t8319: f64, t8322: f64, t8324: f64) -> (f64, f64) {
    let t10408 = t10136 + t10170 + t10200 + t10240 + t10283 + t10323 + t10370 + t10405;
    let t10433 = -0.59049582388386525904e-5_f64 * t8292 - 0.59049582388386525904e-5_f64 * t8298 - 0.86898242813537603826e-4_f64 * t8301 + 0.43449121406768801913e-4_f64 * t8304 + 0.20855578275249024918e-2_f64 * t8306 + 0.12147342662753799615e-3_f64 * t8311 + 0.86898242813537603826e-5_f64 * t8314 + 0.86898242813537603826e-5_f64 * t8317 + 0.41711156550498049836e-2_f64 * t8319 - 0.82402707051983925121e-5_f64 * t8322 - 0.60828769969476322678e-4_f64 * t8324;
    (t10408, t10433)
}
