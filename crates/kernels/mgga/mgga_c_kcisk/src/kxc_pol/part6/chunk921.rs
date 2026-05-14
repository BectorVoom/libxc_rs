//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 921/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk921<F: Float>(t31100: F, t457: F, t2192: F, t7897: F, t5953: F, t1422: F, t1423: F, t30158: F, t2191: F, t26590: F, t5926: F, t5927: F, t7764: F, t13220: F, t13221: F, t30153: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31101 = t457 * t31100;
    let t31106 = t2192 * t7897;
    let t31107 = t5953 * t31106;
    let t31111 = t1422 * t1423 * t30158;
    let t31114 = t26590 * t2191;
    let t31115 = t5926 * t31114;
    let t31118 = t5927 * t7764;
    let t31119 = t5926 * t31118;
    let t31123 = t13220 * t13221 * t30153;
    (t31101, t31106, t31107, t31111, t31114, t31115, t31118, t31119, t31123)
}
