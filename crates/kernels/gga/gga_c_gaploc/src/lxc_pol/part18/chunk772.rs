//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 772/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk772<F: Float>(t7402: F, t7462: F, t7523: F, t7577: F, t7629: F, t7692: F, t7750: F, t7814: F, t2590: F, t747: F, t1961: F, t977: F, t6363: F, t6366: F, t6374: F, t6377: F, t6379: F, t6381: F) -> (F, F, F, F) {
    let t7817 = t7402 + t7462 + t7523 + t7577 + t7629 + t7692 + t7750 + t7814;
    let t7822 = t2590 * t747;
    let t7826 = t977 * t1961;
    let t7851 = 189.0 / 256.0 * t6363 - 483.0 / 8192.0 * t6366 + 147.0 / 524288.0 * t6374 - 49.0 / 524288.0 * t6377 + 161.0 / 8192.0 * t6379 - 63.0 / 256.0 * t6381;
    (t7817, t7822, t7826, t7851)
}
