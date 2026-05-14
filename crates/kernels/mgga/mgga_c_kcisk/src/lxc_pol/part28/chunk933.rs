//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 933/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk933<F: Float>(t2005: F, t7624: F, t12261: F, t2643: F, t782: F, t4419: F, t7639: F, t2009: F, t7586: F, t2630: F, t5483: F, t16672: F, t1993: F, t7528: F, t17016: F, t17054: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18699 = 0.17990788716177317213e-1 * t2005 * t7624;
    let t18700 = t12261 * t2643;
    let t18701 = t782 * t18700;
    let t18703 = t4419 * t7639;
    let t18705 = 0.17990788716177317213e-1 * t782 * t18703;
    let t18711 = 0.47975436576472845902e-1 * t7586 * t2009;
    let t18713 = 0.17990788716177317213e-1 * t2630 * t5483;
    let t18721 = 0.15476481481481481481e-2 * t16672;
    let t18744 = t7528 * t1993;
    let t18751 = 0.15476481481481481481e-2 * t17016;
    let t18766 = 0.23214722222222222222e-2 * t17054;
    (t18699, t18701, t18705, t18711, t18713, t18721, t18744, t18751, t18766)
}
