//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 838/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk838<F: Float>(t10281: F, t501: F, t1853: F, t3432: F, t10667: F, t325: F, t835: F, t3431: F, t723: F, t7290: F, t701: F, t2610: F) -> (F, F, F, F, F, F) {
    let t32100 = t10281 * t501;
    let t32112 = t3432 * t1853;
    let t32179 = t325 * t10667;
    let t32190 = t835 * t10667;
    let t32214 = t3431 * t723;
    let t32215 = t7290 * t32214;
    let t32260 = t3431 * t701;
    let t32261 = t2610 * t32260;
    (t32100, t32112, t32179, t32190, t32215, t32261)
}
