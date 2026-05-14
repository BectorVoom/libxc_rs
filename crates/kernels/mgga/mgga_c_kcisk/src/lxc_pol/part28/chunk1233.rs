//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1233/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1233<F: Float>(t33021: F, t8814: F, t7261: F, t2063: F, t2464: F, t33059: F, t5015: F, t2364: F, t7283: F, t7242: F) -> (F, F, F, F, F, F) {
    let t35111 = t33021 * t8814;
    let t35112 = t7261 * t35111;
    let t35117 = t2063 * t2464;
    let t35118 = t33059 * t35117;
    let t35119 = t5015 * t35118;
    let t35122 = t7283 * t2364;
    let t35123 = t7242 * t35122;
    (t35111, t35112, t35118, t35119, t35122, t35123)
}
