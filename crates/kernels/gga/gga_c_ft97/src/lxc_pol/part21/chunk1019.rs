//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1019/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1019<F: Float>(t1326: F, t8326: F, t1786: F, t5704: F, t8216: F, t38953: F, t5632: F, t5637: F, t8232: F, t1637: F, t5665: F, t5667: F, t1317: F, t5680: F, t38456: F, t91: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t92035 = t8326 * t1326;
    let t92049 = t1786 * t5704;
    let t92055 = t8216 * t1326;
    let t92062 = t38953 * t5632;
    let t92072 = t8232 * t5637;
    let t92140 = t5665 * t1637 * t5667;
    let t92141 = t92140 / 9.0;
    let t92143 = t1317 * t1637 * t5680;
    let t92144 = 4.0 / 9.0 * t92143;
    let t92173 = t91 * t38456;
    (t92035, t92049, t92055, t92062, t92072, t92140, t92141, t92143, t92144, t92173)
}
