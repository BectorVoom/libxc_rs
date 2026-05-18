//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 862/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk862<F: Float>(t2328: F, t2336: F, t2340: F, t6087: F, t6174: F, t6090: F, t6093: F, t6108: F, t6151: F, t6154: F, t6159: F, t6166: F, t6169: F, t6171: F, t6177: F, t6180: F, t6183: F, t6187: F, t6191: F) -> (F, F, F, F, F) {
    let t6243 = F::new(0.17544670867903938621e1) * t2328 * t2336;
    let t6245 = F::new(0.51947577317044391276e2) * t2328 * t2340;
    let t6249 = F::new(0.16068111111111111111e1) * t6087;
    let t6256 = F::new(0.46308888888888888888e0) * t6174;
    let t6262 = F::new(0.264729375e1) * t6151 - F::new(0.52945875e1) * t6154 + F::new(0.3529725e1) * t6159 - t6249 + F::new(0.20659e1) * t6090 - F::new(0.1549425e1) * t6093 + F::new(0.1549425e1) * t6108 - F::new(0.157790625e0) * t6166 + F::new(0.94674375e0) * t6169 + F::new(0.6311625e0) * t6171 - t6256 + F::new(0.104195e1) * t6177 - F::new(0.62517e0) * t6180 - F::new(0.62517e0) * t6183 + F::new(0.937755e0) * t6187 + F::new(0.312585e0) * t6191;
    (t6243, t6245, t6249, t6256, t6262)
}
