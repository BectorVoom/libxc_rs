//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 603/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk603<F: Float>(t1611: F, t2347: F, t240: F, t4535: F, t555: F, t6604: F, t8186: F, t8188: F, t8191: F, t8287: F, t8432: F, t8436: F, t8455: F) -> F {
    let t8459 = t8186 - t8188 + t8191 - t8287 + t240 * (-t1611 * t8455 - F::new(2.0) * t2347 * t6604 + F::new(2.0) * t4535 * t8436 + t555 * t8432 - t8186 + t8188 - t8191 + t8287);
    t8459
}
