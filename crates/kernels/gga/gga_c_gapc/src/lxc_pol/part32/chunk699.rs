//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 699/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk699<F: Float>(t1013: F, t1758: F, t3079: F, t561: F, t1019: F, t1776: F, t19: F, t3071: F, t1971: F, t2993: F, t144: F, t147: F, t200: F, t2999: F, t5319: F, t1338: F, t134: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8830 = t1013 * t1758;
    let t8832 = t561 * t3079;
    let t8833 = t8832 * t1019;
    let t8835 = t1013 * t1776;
    let t8837 = t3071 * t19;
    let t8838 = t1971 * t8837;
    let t8839 = t2993 * t8838;
    let t8840 = t147 * t144;
    let t8841 = t8840 * t200;
    let t8842 = t5319 * t2999;
    let t8843 = t8841 * t8842;
    let t8844 = t8839 * t8843;
    let t8846 = t134 * t1338;
    (t8830, t8832, t8833, t8835, t8837, t8838, t8841, t8843, t8844, t8846)
}
