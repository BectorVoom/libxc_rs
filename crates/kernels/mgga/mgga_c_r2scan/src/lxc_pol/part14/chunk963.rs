//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 963/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk963<F: Float>(t10811: F, t1584: F, t10856: F, t5169: F, t1583: F, t546: F, t2078: F, t3320: F, t783: F, t787: F, t1266: F, t512: F, t57: F, t1607: F, t6271: F, t1615: F, t774: F) -> (F, F, F, F, F, F, F, F) {
    let t37676 = t1584 * t10811;
    let t37681 = t10856 * t5169;
    let t37685 = t546 * t1583;
    let t37696 = t783 * t2078 * t787 * t3320;
    let t37699 = t512 * t1266 * t57;
    let t37700 = t37699 * t1607;
    let t37702 = t10856 * t6271;
    let t37707 = t783 * t774 * t1615 * t3320;
    (t37676, t37681, t37685, t37696, t37699, t37700, t37702, t37707)
}
