//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1107/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1107<F: Float>(t25138: F, t72: F, t1927: F, t6973: F, t6977: F, t2311: F, t76: F, t1926: F, t10298: F, t38: F, t10309: F, t6957: F) -> (F, F, F, F, F, F, F) {
    let t25139 = t25138 * t72;
    let t25140 = t25139 * t1927;
    let t25143 = t6973 * t6977;
    let t25146 = t76 * t2311;
    let t25147 = t1926 * t25146;
    let t25150 = t10298 * t38;
    let t25157 = t10309 * t6957;
    (t25139, t25140, t25143, t25146, t25147, t25150, t25157)
}
