//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1180/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1180<F: Float>(t1894: F, t2563: F, t34159: F, t1869: F, t6961: F, t9687: F, t415: F, t6966: F, t717: F, t1864: F, t2537: F, t1871: F, t2508: F, t1895: F, t1900: F, t2509: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t34160 = t2563 * t1894;
    let t34161 = t34159 * t34160;
    let t34162 = t1869 * t34161;
    let t34164 = t9687 * t6961;
    let t34165 = t415 * t34164;
    let t34167 = t717 * t6966;
    let t34168 = t415 * t34167;
    let t34170 = t1864 * t2537;
    let t34171 = t415 * t34170;
    let t34173 = t2508 * t1871;
    let t34174 = t34173 * t1895;
    let t34175 = t415 * t34174;
    let t34177 = t2509 * t1900;
    (t34160, t34161, t34162, t34164, t34165, t34167, t34168, t34170, t34171, t34173, t34174, t34175, t34177)
}
