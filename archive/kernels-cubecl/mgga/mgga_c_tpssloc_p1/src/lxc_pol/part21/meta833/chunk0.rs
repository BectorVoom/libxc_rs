//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2941/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2941<F: Float>(t13784: F, t17178: F, t2986: F, t10189: F, t5836: F, t2990: F, t17161: F, t17152: F, t48213: F, t17863: F, t42837: F, t10186: F, t17808: F) -> (F, F, F, F, F, F) {
    let t61245 = t2986 * t13784 * t17178;
    let t61250 = t10189 * t5836;
    let t61252 = t2986 * t61250 * t2990;
    let t61258 = t2986 * t13784 * t17161;
    let t61261 = t2986 * t48213 * t17152;
    let t61264 = t2986 * t42837 * t17863;
    let t61273 = t10186 * t17808;
    (t61245, t61252, t61258, t61261, t61264, t61273)
}
