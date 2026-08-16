//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 552/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk552(t1977: f64, t1982: f64, t7428: f64, t1165: f64, t194: f64, t201: f64, t1979: f64, t1987: f64, t2186: f64, t2034: f64, t5016: f64, t2061: f64, t2604: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7430 = t1977 * t7428 * t1982;
    let t7433 = t194 * t1165;
    let t7434 = t7433 * t201;
    let t7436 = t7434 * t1979 * t1982;
    let t7438 = t2186 * t1987;
    let t7440 = t5016 * t2034;
    let t7442 = t2604 * t2061;
    (t7430, t7433, t7434, t7436, t7438, t7440, t7442)
}
