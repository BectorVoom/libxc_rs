//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1373/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1373<F: Float>(t1882: F, t26906: F, t26911: F, t26943: F, t26961: F, t26899: F, t105329: F, t167: F, t2179: F, t2180: F, t23420: F, t26768: F, t3578: F, t446: F, t574: F, t616: F, t6615: F, t95736: F, t95738: F, t95740: F, t95742: F, t95744: F, t95747: F) -> (F,) {
    let t106840 = 4.0 / 9.0 * t1882 * t26906;
    let t106842 = 4.0 / 9.0 * t1882 * t26911;
    let t106844 = 4.0 / 9.0 * t1882 * t26943;
    let t106847 = 2.0 / 9.0 * t1882 * t26961;
    let t106871 = 2.0 / 9.0 * t1882 * t26899;
    let t106872 = -t106840 - t106842 - t106844 + 2.0 / 9.0 * t95736 + t106847 - 2.0 / 3.0 * t446 * t574 * t2179 * t6615 * t2180 - 2.0 / 3.0 * t446 * t574 * t616 * t26768 - t446 * t574 * t167 * t105329 / 3.0 - 8.0 / 27.0 * t95738 - 8.0 / 27.0 * t95740 + 2.0 / 3.0 * t446 * t574 * t3578 * t23420 - 4.0 / 9.0 * t95742 - 2.0 / 9.0 * t95744 - 4.0 / 9.0 * t95747 - t106871;
    (t106872,)
}
