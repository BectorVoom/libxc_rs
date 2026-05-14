//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 976/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk976<F: Float>(t16004: F, t7234: F, t10879: F, t2459: F, t1773: F, t4989: F, t7230: F, t10798: F, t7257: F, t5013: F, t1774: F, t963: F, t1782: F, t1785: F, t6714: F, t16026: F, t7242: F) -> (F, F, F, F, F, F, F, F) {
    let t17321 = t7234 * t16004;
    let t17326 = t10879 * t2459;
    let t17327 = t1773 * t17326;
    let t17330 = 0.11993859144118211475e-1 * t4989 * t7230;
    let t17333 = t10798 * t7257;
    let t17335 = 0.11993859144118211475e-1 * t5013 * t17333;
    let t17336 = t963 * t1774;
    let t17337 = t17336 * t1782;
    let t17338 = t6714 * t1785;
    let t17339 = t17337 * t17338;
    let t17342 = t7242 * t16026;
    (t17321, t17327, t17330, t17335, t17336, t17337, t17339, t17342)
}
