//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1307/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1307<F: Float>(t109152: F, t109154: F, t109160: F, t109162: F, t109165: F, t110821: F, t110858: F, t110892: F, t110898: F, t111196: F, t111201: F, t111203: F, t111206: F, t111221: F, t111521: F, t1140: F, t1147: F, t15713: F, t15716: F, t15723: F, t15724: F, t15727: F, t233: F, t2705: F, t32543: F, t32546: F, t32549: F, t32560: F, t32579: F, t3442: F, t3443: F, t3460: F, t43184: F, t44181: F, t9404: F) -> (F,) {
    let t111524 = t233 * (6.0 * t32549 * t15727 - t109152 + t109154 + t109160 + 6.0 * t110821 * t3443 - t1140 * (t110858 + t110892) - 18.0 * t15723 * t9404 * t3443 - t109162 + t109165 - 3.0 * t110898 * t1147 + t111196 + 6.0 * t3442 * t32579 * t1147 + 24.0 * t43184 * t2705 * t15724 - 3.0 * t15713 * t9404 + 6.0 * t15716 * t32546 - 18.0 * t44181 * t32543 - 3.0 * t32560 * t3460 + t111201 - t111203 - t111206 + t111221 + t111521);
    (t111524,)
}
