//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1212/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1212<F: Float>(t1385: F, t1889: F, t1938: F, t94228: F, t833: F, t27370: F, t5732: F, t5885: F, t28335: F, t28369: F, t102085: F, t102102: F, t102106: F, t102109: F, t103191: F, t27369: F, t28348: F, t52697: F, t5440: F, t94626: F, t98155: F, t98255: F, t98270: F, t98653: F) -> (F, F, F, F) {
    let t103328 = t94228 * t1889 * t1938 * t1385;
    let t103331 = t1938 * t833;
    let t103340 = t27370 * t5885 * t5732;
    let t103343 = t28369 * t28335;
    let t103347 = t98255 + 0.4946917361111111111e-3 * t98155 * t28348 + 0.3684876543209876543e-2 * t102085 - 0.92673611111111111112e-3 * t94626 * t98653 * t1889 * t52697 - 0.46336805555555555556e-3 * t94626 * t103328 - 0.92673611111111111112e-3 * t94626 * t98270 * t5440 * t103331 - 0.55273148148148148147e-3 * t102102 - 0.18550940104166666667e-3 * t27369 * t103191 - 0.18550940104166666667e-3 * t27369 * t103340 - 0.15445601851851851852e-3 * t103343 + 0.99491666666666666664e-2 * t102106 - 0.33163888888888888888e-2 * t102109;
    (t103328, t103331, t103340, t103347)
}
