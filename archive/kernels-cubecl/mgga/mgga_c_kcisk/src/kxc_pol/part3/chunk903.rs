//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 903/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk903<F: Float>(t13456: F, t5634: F, t3484: F, t5633: F, t13299: F, t3796: F, t3936: F, t3959: F, t1163: F, t3961: F, t1322: F, t3579: F) -> (F, F, F, F) {
    let t13464 = t5634 * t13456;
    let t13465 = t3484 * t13464;
    let t13466 = t5633 * t13465;
    let t13468 = t5634 * t13299;
    let t13469 = t3796 * t13468;
    let t13470 = t5633 * t13469;
    let t13472 = t3936 * t3959;
    let t13473 = t1163 * t3961;
    let t13474 = t13472 * t13473;
    let t13477 = t3579 * t1322;
    (t13466, t13470, t13474, t13477)
}
