//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1081/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1081<F: Float>(t2206: F, t2394: F, t2211: F, t2254: F, t102: F, t327: F, t959: F, t285: F, t6849: F, t2762: F, t328: F, t332: F) -> (F, F, F, F, F) {
    let t18551 = t2394 * t2206;
    let t18553 = t2211 * t2254;
    let t18639 = t102 * t327 * t959;
    let t18679 = F::new(1.0) / t6849 / t285;
    let t18680 = F::new(1.0) / t2762 / t328 * t332 * t18679;
    (t18551, t18553, t18639, t18679, t18680)
}
