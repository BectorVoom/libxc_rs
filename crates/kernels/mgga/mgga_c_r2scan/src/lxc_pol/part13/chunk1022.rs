//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1022/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1022<F: Float>(t10698: F, t2593: F, t11805: F, t37641: F, t1054: F, t5108: F, t7352: F, t37759: F, t37749: F, t37762: F, t39655: F, t39658: F, t39661: F, t39664: F, t39667: F, t39669: F) -> (F,) {
    let t39672 = t10698 * t2593;
    let t39673 = 0.25610080155860322884e0 * t39672;
    let t39674 = t37641 * t11805;
    let t39677 = t5108 * t1054 * t7352;
    let t39679 = 0.11902492299418487743e0 * t37759;
    let t39681 = -0.65495539973149862688e-2 * t39655 + 0.43663693315433241792e-2 * t39658 - 0.13002332610081402845e0 * t39661 - 0.17336443480108537126e0 * t39664 + 0.65495539973149862688e-2 * t39667 - 0.43341108700271342816e-1 * t39669 - 0.69345773920434148506e0 * t37749 - t39673 + 0.2600466522016280569e0 * t39674 - 0.2600466522016280569e0 * t39677 - t39679 + 0.23804984598836975486e-2 * t37762;
    (t39681,)
}
