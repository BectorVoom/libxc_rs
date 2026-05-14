//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1055/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1055<F: Float>(t18203: F, t18223: F, t18243: F, t18731: F, t18755: F, t18783: F, t18810: F, t18834: F, t2049: F, t7690: F, t12345: F, t12352: F, t17777: F, t17781: F, t17786: F, t17789: F, t17792: F, t18176: F, t18179: F, t18182: F, t5532: F, t7659: F, t802: F) -> (F, F, F) {
    let t18837 = t18203 + t18223 + t18243 + t18731 + t18755 + t18783 + t18810 + t18834;
    let t18839 = t7690 * t2049;
    let t18842 = 4.0 * t12345 * t7659 - 6.0 * t12352 * t18182 - 2.0 * t18179 * t2049 + t18837 * t802 + 4.0 * t18839 * t5532 - t17777 - t17781 + t17786 - t17789 - t17792 + t18176;
    (t18837, t18839, t18842)
}
