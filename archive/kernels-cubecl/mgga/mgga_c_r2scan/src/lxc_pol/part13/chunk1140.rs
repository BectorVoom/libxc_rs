//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1140/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1140<F: Float>(t10814: F, t2651: F, t10698: F, t2593: F, t11805: F, t37641: F, t1054: F, t5108: F, t7352: F, t37759: F, t37749: F, t37762: F, t39655: F, t39658: F, t39661: F, t39664: F, t39667: F) -> F {
    let t39669 = t2651 * t10814;
    let t39672 = t10698 * t2593;
    let t39673 = F::cast_from(0.25610080155860322884e0_f64) * t39672;
    let t39674 = t37641 * t11805;
    let t39677 = t5108 * t1054 * t7352;
    let t39679 = F::cast_from(0.11902492299418487743e0_f64) * t37759;
    let t39681 = -F::cast_from(0.65495539973149862688e-2_f64) * t39655 + F::cast_from(0.43663693315433241792e-2_f64) * t39658 - F::cast_from(0.13002332610081402845e0_f64) * t39661 - F::cast_from(0.17336443480108537126e0_f64) * t39664 + F::cast_from(0.65495539973149862688e-2_f64) * t39667 - F::cast_from(0.43341108700271342816e-1_f64) * t39669 - F::cast_from(0.69345773920434148506e0_f64) * t37749 - t39673 + F::cast_from(0.2600466522016280569e0_f64) * t39674 - F::cast_from(0.2600466522016280569e0_f64) * t39677 - t39679 + F::cast_from(0.23804984598836975486e-2_f64) * t37762;
    t39681
}
