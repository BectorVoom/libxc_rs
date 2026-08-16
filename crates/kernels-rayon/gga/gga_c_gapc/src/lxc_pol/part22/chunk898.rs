//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 898/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk898(t10236: f64, t827: f64, t1063: f64, t10201: f64, t10205: f64, t10208: f64, t10213: f64, t10217: f64, t10220: f64, t10223: f64, t10227: f64, t10232: f64, t10234: f64) -> (f64, f64, f64) {
    let t10237 = t10236 * t827;
    let t10238 = t10237 * t1063;
    let t10240 = -0.74372214241464483348e-4_f64 * t10201 + 0.11742981196020707897e-4_f64 * t10205 + 0.58714905980103539485e-5_f64 * t10208 + 0.56366309740899397906e-3_f64 * t10213 - 0.33406432906439709826e-4_f64 * t10217 - 0.58714905980103539485e-5_f64 * t10220 - 0.342503618217270647e-5_f64 * t10223 - 0.342503618217270647e-5_f64 * t10227 - 0.20299047773010240345e-6_f64 * t10232 - 0.11742981196020707897e-4_f64 * t10234 - 0.58714905980103539485e-5_f64 * t10238;
    (t10237, t10238, t10240)
}
