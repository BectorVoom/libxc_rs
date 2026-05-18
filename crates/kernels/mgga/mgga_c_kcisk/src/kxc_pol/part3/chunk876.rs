//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 876/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk876<F: Float>(t13143: F, t3544: F, t306: F, t3529: F, t459: F, t1175: F, t3575: F, t3530: F, t425: F, t1364: F, t12983: F, t5895: F) -> (F, F, F, F) {
    let t13144 = t3544 * t13143;
    let t13148 = t3529 * t306 * t459;
    let t13149 = t3575 * t1175;
    let t13150 = t13148 * t13149;
    let t13153 = t3530 * t425;
    let t13154 = t3575 * t1364;
    let t13155 = t13153 * t13154;
    let t13158 = t5895 * t12983;
    (t13144, t13150, t13155, t13158)
}
