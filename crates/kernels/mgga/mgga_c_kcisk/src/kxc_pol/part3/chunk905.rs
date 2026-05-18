//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 905/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk905<F: Float>(t1322: F, t3583: F, t3937: F, t1163: F, t3988: F, t1319: F, t6174: F, t3575: F, t12868: F, t6183: F, t4092: F, t45: F) -> (F, F, F, F, F) {
    let t13496 = t3583 * t1322;
    let t13497 = t3937 * t13496;
    let t13500 = t1163 * t3988;
    let t13501 = t3937 * t13500;
    let t13504 = t6174 * t1319;
    let t13505 = t3575 * t1322;
    let t13506 = t13504 * t13505;
    let t13509 = t6183 * t12868;
    let t13512 = t45 * t4092;
    (t13497, t13501, t13506, t13509, t13512)
}
