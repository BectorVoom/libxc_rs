//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 798/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk798<F: Float>(t1611: F, t1620: F, t240: F, t2748: F, t4530: F, t4535: F, t555: F, t9482: F, t9484: F, t9485: F, t9488: F, t9510: F, t9555: F, t9557: F, t9560: F, t9571: F) -> (F,) {
    let t9575 = t9482 - t9484 - t9485 + t9488 - t9510 + t240 * (-t1611 * t9571 - t1620 * t9557 - t2748 * t4530 + 2.0 * t4535 * t9560 + t555 * t9555 - t9482 + t9484 + t9485 - t9488 + t9510);
    (t9575,)
}
