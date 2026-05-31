//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 530/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk530<F: Float>(t4368: F, t79: F, t534: F, t1587: F, t538: F, t398: F, t1591: F, t1586: F, t1579: F, t3969: F, t1582: F, t3973: F) -> (F, F, F, F, F, F, F, F) {
    let t4369 = t79 * t4368;
    let t4370 = t4369 * t534;
    let t4374 = F::cast_from(1.0_f64) / t1587 / t538;
    let t4375 = t398 * t4374;
    let t4376 = t1591 * t1591;
    let t4377 = t4375 * t4376;
    let t4378 = t1586 * t4377;
    let t4381 = t1579 * t3969;
    let t4384 = t3973 * t1582;
    (t4369, t4370, t4374, t4376, t4377, t4378, t4381, t4384)
}
