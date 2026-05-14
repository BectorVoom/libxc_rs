//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 345/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk345<F: Float>(t1642: F, t1667: F, t1671: F, t1686: F, t1961: F, t1966: F, t1975: F, t1979: F, t1980: F, t1987: F, t240: F, t764: F) -> (F,) {
    let t1990 = -t1642 + t1667 + t240 * (-0.3109e-1 * t1961 * t764 + 1.0 * t1966 * t1975 + t1642 - t1667 - 0.19751789702565206229e-1 * t1671 + 0.58482233974552040708e0 * t1979 * t1980) + 0.19751789702565206229e-1 * t240 * t1671 - 0.58482233974552040708e0 * t1987 * t1686;
    (t1990,)
}
