//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 974/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk974<F: Float>(t2488: F, t6790: F, t7055: F, t3521: F, t8896: F, t1876: F, t1877: F, t22591: F, t8908: F, t8912: F, t8920: F, t682: F, t8536: F, t1648: F, t4629: F, t1824: F, t8537: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22642 = t2488 * t6790;
    let t22643 = t7055 * t22642;
    let t22646 = t3521 * t8896;
    let t22649 = t1876 * t1877 * t22591;
    let t22652 = t3521 * t8908;
    let t22654 = t3521 * t8912;
    let t22656 = t3521 * t8920;
    let t22658 = t682 * t8536;
    let t22659 = t22658 * t1648;
    let t22660 = t4629 * t22659;
    let t22663 = t8537 * t1824;
    (t22642, t22643, t22646, t22649, t22652, t22654, t22656, t22659, t22660, t22663)
}
