//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 378/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk378<F: Float>(t1973: F, t1974: F, t1676: F, t591: F, t1683: F, t1685: F, t240: F, t1642: F, t1667: F, t1671: F, t1686: F, t1961: F, t1966: F, t764: F, t794: F, t772: F) -> (F, F, F, F, F, F, F, F) {
    let t1975 = t1973 * t1974;
    let t1979 = t591 * t1676;
    let t1980 = t1683 * t1685;
    let t1987 = t240 * t591;
    let t1990 = -t1642 + t1667 + t240 * (-0.3109e-1 * t1961 * t764 + 1.0 * t1966 * t1975 + t1642 - t1667 - 0.19751789702565206229e-1 * t1671 + 0.58482233974552040708e0 * t1979 * t1980) + 0.19751789702565206229e-1 * t240 * t1671 - 0.58482233974552040708e0 * t1987 * t1686;
    let t1992 = t794 * t794;
    let t1993 = 1.0 / t1992;
    let t1994 = t772 * t1993;
    (t1975, t1979, t1980, t1987, t1990, t1992, t1993, t1994)
}
