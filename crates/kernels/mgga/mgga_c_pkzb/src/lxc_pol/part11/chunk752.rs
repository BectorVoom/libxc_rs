//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 752/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk752<F: Float>(t204: F, t2739: F, t648: F, t5528: F, t972: F, t1837: F, t8: F, t1066: F, t1281: F) -> (F, F, F, F, F) {
    let t7335 = t204 * t648 * t2739;
    let t7336 = 0.103295e1 * t7335;
    let t7337 = t5528 * t972;
    let t7340 = t1837 * t8;
    let t7357 = t204 * t1281 * t1066;
    (t7335, t7336, t7337, t7340, t7357)
}
