//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 684/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk684<F: Float>(t10449: F, t682: F, t11385: F, t1814: F, t1060: F, t4658: F, t5101: F, t5100: F, t680: F, t11394: F, t1824: F, t4684: F, t1850: F, t5090: F, t11460: F, t11461: F, t11463: F, t11465: F, t11467: F, t1809: F, t674: F) -> (F,) {
    let t11469 = t682 * t10449;
    let t11472 = t1814 * t11385;
    let t11476 = t5101 * t1060 * t4658;
    let t11480 = 1.0 / t5100 / t680;
    let t11481 = t11480 * t11394;
    let t11485 = t5101 * t1824 * t4684;
    let t11488 = t1850 * t5090;
    let t11490 = -t11460 - 0.14055920378328537299e-1 * t11461 - 0.28111840756657074597e-1 * t11463 + 0.70279601891642686494e-2 * t11465 + 0.14055920378328537299e-1 * t11467 - 0.23426533963880895498e-2 * t1809 * t11469 - 0.46853067927761790996e-2 * t674 * t11472 - 0.42167761134985611897e-1 * t1809 * t11476 - 0.56223681513314149196e-1 * t674 * t11481 + 0.42167761134985611897e-1 * t674 * t11485 - 0.14055920378328537299e-1 * t11488;
    (t11490,)
}
