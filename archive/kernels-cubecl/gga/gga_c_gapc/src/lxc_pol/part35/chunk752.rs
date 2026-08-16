//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 752/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk752<F: Float>(t200: F, t8840: F, t2999: F, t5319: F, t8839: F, t1338: F, t134: F, t647: F, t2998: F, t2996: F, t1030: F, t8838: F) -> (F, F, F, F, F, F) {
    let t8841 = t8840 * t200;
    let t8842 = t5319 * t2999;
    let t8843 = t8841 * t8842;
    let t8844 = t8839 * t8843;
    let t8846 = t134 * t1338;
    let t8847 = t647 * t8846;
    let t8848 = t2998 * t8847;
    let t8849 = t2996 * t8848;
    let t8851 = t1030 * t8838;
    (t8841, t8843, t8844, t8848, t8849, t8851)
}
