//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1131/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1131<F: Float>(t4474: F, t58: F, t428: F, t4417: F, t100737: F, t100843: F, t100980: F, t101026: F, t115617: F, t15689: F, t15777: F, t15819: F, t15822: F, t15825: F, t1669: F, t1701: F, t1742: F, t22522: F, t22591: F, t22652: F, t22696: F, t22701: F, t22796: F, t22797: F, t22798: F, t22826: F, t25734: F, t29514: F, t29520: F, t38013: F, t4491: F, t5569: F, t5570: F, t5571: F, t7889: F, t92797: F, t92920: F) -> (F, F, F) {
    let t115815 = t58 * t4474;
    let t115820 = t4417 * t428;
    let t115855 = -0.77462893625097599762e-3 * t22826 * t101026 * t15689 + 0.13336606457645654222e-1 * t38013 * t22591 * t115815 * t428 - 0.25537443351851851852e-1 * t22522 * t5570 * t1742 * t115820 + 0.3443640424494650102e-5 * t100843 * t100737 * t115617 + 0.22270151833971792333e-3 * t5569 * t5570 * t5571 * t15777 + 4.0 * t22696 * t29520 + 4.0 * t1669 * t92920 * t4474 - 0.85124811172839506174e-2 * t92797 + 0.52801466802079540469e-5 * t22796 * t22797 * t29514 * t22798 - 0.46509801892875584e-2 * t22826 * t15819 - 0.558117622714507008e-2 * t25734 * t15822 + 0.93019603785751168e-2 * t25734 * t15825 + 0.23709522591370051951e-1 * t7889 * t1701 * t22652 * t4474 + t100980 - 2.0 * t1669 * t22701 * t4491;
    (t115815, t115820, t115855)
}
