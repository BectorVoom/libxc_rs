//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 385/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk385<F: Float>(t1973: F, t1974: F, t1676: F, t591: F, t1683: F, t1685: F, t240: F, t1642: F, t1667: F, t1671: F, t1686: F, t1961: F, t1966: F, t764: F) -> (F, F, F, F, F) {
    let t1975 = t1973 * t1974;
    let t1979 = t591 * t1676;
    let t1980 = t1683 * t1685;
    let t1987 = t240 * t591;
    let t1990 = -t1642 + t1667 + t240 * (-F::cast_from(0.3109e-1_f64) * t1961 * t764 + F::cast_from(1.0_f64) * t1966 * t1975 + t1642 - t1667 - F::cast_from(0.19751789702565206229e-1_f64) * t1671 + F::cast_from(0.58482233974552040708e0_f64) * t1979 * t1980) + F::cast_from(0.19751789702565206229e-1_f64) * t240 * t1671 - F::cast_from(0.58482233974552040708e0_f64) * t1987 * t1686;
    (t1975, t1979, t1980, t1987, t1990)
}
