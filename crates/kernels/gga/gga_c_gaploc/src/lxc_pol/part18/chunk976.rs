//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 976/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk976<F: Float>(t10619: F, t10611: F, t10612: F, t10617: F, t1429: F, t9550: F, t9554: F, t9557: F, t9560: F, t9564: F, t9569: F, t9571: F, t9575: F, t9577: F, t9579: F, t9582: F, t9584: F) -> F {
    let t10620 = F::cast_from(0.14896037479937677779e-1_f64) * t10619;
    let t10621 = -t10611 + F::cast_from(0.39722766613167140743e-1_f64) * t1429 * t10612 - t10617 + t10620 + t9550 - t9554 + t9557 + t9560 - t9564 - t9569 - t9571 - t9575 + t9577 + t9579 + t9582 - t9584;
    t10621
}
