//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 862/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk862(t2328: f64, t2336: f64, t2340: f64, t6087: f64, t6174: f64, t6090: f64, t6093: f64, t6108: f64, t6151: f64, t6154: f64, t6159: f64, t6166: f64, t6169: f64, t6171: f64, t6177: f64, t6180: f64, t6183: f64, t6187: f64, t6191: f64) -> (f64, f64, f64, f64, f64) {
    let t6243 = 0.17544670867903938621e1_f64 * t2328 * t2336;
    let t6245 = 0.51947577317044391276e2_f64 * t2328 * t2340;
    let t6249 = 0.16068111111111111111e1_f64 * t6087;
    let t6256 = 0.46308888888888888888e0_f64 * t6174;
    let t6262 = 0.264729375e1_f64 * t6151 - 0.52945875e1_f64 * t6154 + 0.3529725e1_f64 * t6159 - t6249 + 0.20659e1_f64 * t6090 - 0.1549425e1_f64 * t6093 + 0.1549425e1_f64 * t6108 - 0.157790625e0_f64 * t6166 + 0.94674375e0_f64 * t6169 + 0.6311625e0_f64 * t6171 - t6256 + 0.104195e1_f64 * t6177 - 0.62517e0_f64 * t6180 - 0.62517e0_f64 * t6183 + 0.937755e0_f64 * t6187 + 0.312585e0_f64 * t6191;
    (t6243, t6245, t6249, t6256, t6262)
}
