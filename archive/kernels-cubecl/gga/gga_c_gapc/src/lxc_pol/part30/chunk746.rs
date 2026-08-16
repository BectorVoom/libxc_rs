//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 746/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk746<F: Float>(t19: F, t3071: F, t1971: F, t2993: F, t144: F, t147: F, t200: F, t2999: F, t5319: F, t1338: F, t134: F, t647: F) -> (F, F, F, F, F, F) {
    let t8837 = t3071 * t19;
    let t8838 = t1971 * t8837;
    let t8839 = t2993 * t8838;
    let t8840 = t147 * t144;
    let t8841 = t8840 * t200;
    let t8842 = t5319 * t2999;
    let t8843 = t8841 * t8842;
    let t8844 = t8839 * t8843;
    let t8846 = t134 * t1338;
    let t8847 = t647 * t8846;
    (t8837, t8838, t8841, t8843, t8844, t8847)
}
