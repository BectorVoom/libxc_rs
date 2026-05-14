//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1306/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1306<F: Float>(t10340: F, t31862: F, t10337: F, t2692: F, t43141: F, t10336: F, t1053: F, t32552: F, t3186: F, t3274: F, t9358: F, t111223: F, t111224: F, t111472: F, t111507: F, t15723: F, t15760: F, t3460: F, t44167: F, t9392: F, t9395: F) -> (F, F, F, F, F) {
    let t111509 = 12.0 * t10340 * t31862;
    let t111512 = 24.0 * t43141 * t2692 * t10337;
    let t111515 = 18.0 * t10336 * t32552 * t1053;
    let t111518 = 6.0 * t3186 * t9358 * t3274;
    let t111521 = -18.0 * t15723 * t3460 * t9395 - t15760 * t9392 + 6.0 * t44167 * t9395 + t111223 + t111224 - t111472 + t111507 - t111509 - t111512 + t111515 - t111518;
    (t111509, t111512, t111515, t111518, t111521)
}
